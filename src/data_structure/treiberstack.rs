use crossbeam::epoch::{self, Atomic, CompareExchangeError, Owned};
use std::{mem::ManuallyDrop, sync::atomic::Ordering};

struct Node<T> {
    val: ManuallyDrop<T>,
    next: Atomic<Node<T>>,
}

pub struct TreiberStack<T> {
    head: Atomic<Node<T>>,
}

impl<T> TreiberStack<T> {
    pub fn new() -> Self {
        Self {
            head: Atomic::null(),
        }
    }

    pub fn push(&self, val: T) {
        let mut owned = Owned::new(Node {
            val: ManuallyDrop::new(val),
            next: Atomic::null(),
        });

        let guard = epoch::pin();

        loop {
            let head = self.head.load(Ordering::Relaxed, &guard);
            owned.next.store(head, Ordering::Relaxed);
            match self.head.compare_exchange(head, owned, Ordering::Release, Ordering::Relaxed, &guard) {
                Ok(_) => return,
                Err(n) => owned = n.new,
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        let guard = epoch::pin();
        loop {
            let head = self.head.load(Ordering::Acquire, &guard);

            match unsafe { head.as_ref() } {
                Some(h) => {
                    let next = h.next.load(Ordering::Relaxed, &guard);

                    if self
                        .head
                        .compare_exchange(head, next, Ordering::Release, Ordering::Relaxed, &guard)
                        .is_ok()
                    {
                        let val = unsafe { (&h.val as *const ManuallyDrop<T> as *const T).read() };
                        unsafe {
                            guard.defer_destroy(head);
                        }
                        return Some(val);
                    }
                }
                None => return None,
            }
        }
    }
}

impl<T> Default for TreiberStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for TreiberStack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let guard = epoch::pin();
        let top = self.head.load(Ordering::Relaxed, &guard);

        unsafe { top.as_ref() }.map(|top_ptr| {
            let next = top_ptr.next.load(Ordering::Relaxed, &guard);
            self.head.store(next, Ordering::Relaxed);
            let res = unsafe { (&top_ptr.val as *const ManuallyDrop<T> as *const T).read() };
            unsafe {
                guard.defer_destroy(top);
            }
            res
        })
    }
}

impl<T> Drop for TreiberStack<T> {
    fn drop(&mut self) {
        for _ in self.by_ref() {}
    }
}

#[cfg(test)]
mod test {
    use super::TreiberStack;

    #[test]
    fn push_and_pop() {
        let stack = TreiberStack::new();
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));
    }

    #[test]
    fn is_sync() {
        fn test_sync(_: impl Sync) {}
        let stack = TreiberStack::<i32>::new();
        test_sync(stack);
    }

    #[test]
    fn is_send() {
        fn test_send(_: impl Send) {}
        let stack = TreiberStack::<i32>::new();
        test_send(stack);
    }
}
