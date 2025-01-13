use std::{
    cell::UnsafeCell,
    future::Future,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
    task::{Poll, Waker},
};

use crate::data_structure::queue::Queue;

pub struct AsyncLock {
    inner: AtomicBool,
    // Wrap each waker with `Option`, because `Queue` needs elem to be `Removeable`.
    wakers: Queue<Option<Waker>>,
}

unsafe impl Sync for AsyncLock {}

struct AsyncLockFut<'a> {
    lock: &'a AsyncLock,
}

impl<'a> Future for AsyncLockFut<'a> {
    type Output = AsyncLockGuard<'a>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if let Ok(false) =
            self.lock
                .inner
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
        {
            Poll::Ready(AsyncLockGuard::new(self.lock))
        } else {
            self.lock.wakers.push(cx.waker().clone().into());

            // Ensure there is at least one task can wake up this task.
            if !self.lock.inner.load(Ordering::Acquire) {
                if let Some(waker) = self.lock.wakers.pop() {
                    waker.unwrap().wake();
                }
            }

            Poll::Pending
        }
    }
}

struct AsyncLockGuard<'a> {
    lock: &'a AsyncLock,
}

impl<'a> AsyncLockGuard<'a> {
    fn new(lock: &'a AsyncLock) -> Self {
        Self { lock }
    }
}

impl Drop for AsyncLockGuard<'_> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

impl AsyncLock {
    pub fn new() -> Self {
        Self {
            inner: AtomicBool::new(false),
            wakers: Queue::new(),
        }
    }

    pub async fn lock(&self) -> AsyncLockGuard {
        AsyncLockFut { lock: self }.await
    }

    pub fn unlock(&self) {
        if let Ok(true) =
            self.inner
                .compare_exchange(true, false, Ordering::AcqRel, Ordering::Relaxed)
        {
            while let Some(waker) = self.wakers.pop() {
                waker.unwrap().wake();
            }
        } else {
            panic!()
        }
    }
}

pub struct AsyncMutex<T> {
    inner: UnsafeCell<T>,
    lock: AsyncLock,
}

unsafe impl<T> Sync for AsyncMutex<T> {}

pub struct AsyncMutexGuard<'a, T> {
    mutex: &'a AsyncMutex<T>,
    _guard: AsyncLockGuard<'a>,
}

impl<T> Deref for AsyncMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.inner.get() }
    }
}

impl<T> DerefMut for AsyncMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.inner.get() }
    }
}

impl<T> AsyncMutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            inner: UnsafeCell::new(data),
            lock: AsyncLock::new(),
        }
    }

    pub async fn lock(&self) -> AsyncMutexGuard<T> {
        let lock_guard = self.lock.lock().await;
        AsyncMutexGuard {
            mutex: self,
            _guard: lock_guard,
        }
    }
}
