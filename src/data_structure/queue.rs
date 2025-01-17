use std::{
    mem::{ManuallyDrop, MaybeUninit},
    sync::atomic::{AtomicBool, Ordering},
};

use crossbeam::epoch::{self, Atomic, Guard, Owned, Shared};

struct Removable<T> {
    val: ManuallyDrop<T>,
    present: AtomicBool,
}

impl<T> Removable<T> {
    pub fn new(val: T) -> Self {
        Self {
            val: ManuallyDrop::new(val),
            present: AtomicBool::new(true),
        }
    }

    pub fn empty() -> Self {
        Self {
            #[allow(clippy::uninit_assumed_init)]
            val: ManuallyDrop::new(unsafe { MaybeUninit::uninit().assume_init() }),
            present: AtomicBool::new(false),
        }
    }

    pub fn take(&self) -> Option<T> {
        if self
            .present
            .compare_exchange(true, false, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            Some(unsafe { (&self.val as *const ManuallyDrop<T> as *const T).read() })
        } else {
            None
        }
    }
}

struct Node<T> {
    val: Removable<T>,
    next: Atomic<Node<T>>,
}

pub struct Queue<T> {
    front: Atomic<Node<T>>,
    back: Atomic<Node<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let sentinel = Atomic::new(Node {
            val: Removable::empty(),
            next: Atomic::null(),
        });

        Self {
            front: sentinel.clone(),
            back: sentinel,
        }
    }

    pub fn push(&self, val: T) {
        let guard = epoch::pin();
        let owned = Owned::new(Node {
            val: Removable::new(val),
            next: Atomic::null(),
        });

        let p = owned.into_shared(&guard);
        let prev_back = self.back.swap(p, Ordering::AcqRel, &guard);
        unsafe {
            prev_back.as_ref().unwrap().next.store(p, Ordering::Release);
        }
    }

    pub fn pop(&self) -> Option<T> {
        let guard = epoch::pin();

        loop {
            let front = self.front.load(Ordering::Relaxed, &guard);
            let head = unsafe { front.as_ref().unwrap() };

            match head.val.take() {
                Some(r) => {
                    unsafe { self.try_discard_first_and_move_on(front, &guard) };
                    break Some(r);
                }
                None => {
                    if unsafe { !self.try_discard_first_and_move_on(front, &guard) } {
                        break None;
                    }
                }
            }
        }
    }

    unsafe fn try_discard_first_and_move_on(
        &self,
        prev_first: Shared<Node<T>>,
        guard: &Guard,
    ) -> bool {
        let next = prev_first.as_ref().unwrap().next.clone();
        let next = next.load(Ordering::Acquire, guard);

        if next.is_null() {
            return false;
        }

        if self
            .front
            .compare_exchange(prev_first, next, Ordering::Relaxed, Ordering::Relaxed, guard)
            .is_ok()
        {
            guard.defer_destroy(prev_first);
        }

        true
    }
}

impl<T> Iterator for Queue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let guard = epoch::pin();
        loop {
            let head = self.front.load(Ordering::Relaxed, &guard);
            let head_ptr = unsafe { head.as_ref().expect("Queue has no node!") };

            let next = head_ptr.next.load(Ordering::Relaxed, &guard);
            match head_ptr.val.take() {
                Some(val) => {
                    if !next.is_null() {
                        unsafe {
                            guard.defer_destroy(head);
                        }
                        self.front.store(next, Ordering::Relaxed);
                    }

                    break Some(val);
                }
                None => {
                    if next.is_null() {
                        break None;
                    }

                    unsafe {
                        guard.defer_destroy(head);
                    }
                    self.front.store(next, Ordering::Relaxed);
                }
            }
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        let guard = epoch::pin();
        let mut head = self.front.load(Ordering::Relaxed, &guard);
        while let Some(head_ptr) = unsafe { head.as_ref() } {
            let val = &head_ptr.val;
            let _ = val.take();

            unsafe {
                guard.defer_destroy(head);
            }

            head = head_ptr.next.load(Ordering::Relaxed, &guard);
        }
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn push_and_pop() {
        let queue = Queue::new();
        for i in 0..5 {
            queue.push(i);
        }

        for i in 0..5 {
            assert_eq!(queue.pop(), Some(i));
        }
    }

    #[test]
    fn pop_empty() {
        let queue = Queue::new();
        assert_eq!(queue.pop(), None);
        queue.push(1);
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn iter() {
        let queue = Queue::new();
        for i in 0..4 {
            queue.push(i);
        }

        for (i, v) in queue.enumerate() {
            assert_eq!(v, i);
        }
    }
}
