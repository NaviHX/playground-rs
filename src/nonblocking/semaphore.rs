use crate::data_structure::queue::Queue;
use std::{
    future::Future,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Poll, Waker},
};

pub struct AsyncSemaphore {
    val: AtomicUsize,
    queue: Queue<Option<Waker>>,
}

impl AsyncSemaphore {
    pub fn new(init: usize) -> Self {
        Self {
            val: AtomicUsize::new(init),
            queue: Queue::new(),
        }
    }

    unsafe fn up(&self) {
        self.val.fetch_add(1, Ordering::Release);

        while let Some(task) = self.queue.pop() {
            task.unwrap().wake();
        }
    }

    pub async fn down(&self) -> AsyncSemaphoreGuard {
        DownFut { semaphore: self }.await;
        AsyncSemaphoreGuard { semaphore: self }
    }
}

struct DownFut<'a> {
    semaphore: &'a AsyncSemaphore,
}

impl Future for DownFut<'_> {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        loop {
            let val = self.semaphore.val.load(Ordering::Acquire);

            if val > 0 {
                let new = val - 1;
                if self
                    .semaphore
                    .val
                    .compare_exchange(val, new, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
                    break Poll::Ready(());
                }
            } else {
                let waker = cx.waker().clone();
                self.semaphore.queue.push(Some( waker ));

                if self.semaphore.val.load(Ordering::Acquire) > 0 {
                    if let Some(waker) = self.semaphore.queue.pop() {
                        waker.unwrap().wake();
                    }
                }

                break Poll::Pending;
            }
        }
    }
}

pub struct AsyncSemaphoreGuard<'a> {
    semaphore: &'a AsyncSemaphore,
}

impl Drop for AsyncSemaphoreGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            self.semaphore.up();
        }
    }
}

#[cfg(test)]
mod test {

}
