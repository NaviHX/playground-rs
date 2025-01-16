use std::{ops::{Deref, DerefMut}, ptr::NonNull};

macro_rules! LeType {
    ($left:expr, $right:expr) => {
        [(); $right - $left]
    };
}

macro_rules! EqType {
    ($left:expr, $right:expr) => {
        (LeType!($left, $right), LeType!($right, $left))
    }
}

pub struct Trc<T, const NUM: usize, const DEN: usize> {
    inner: NonNull<T>,
}

/// Impl for both sharing and unique references.
impl<T, const NUM: usize, const DEN: usize> Trc<T, NUM, DEN>
where
    LeType!(NUM, DEN): Sized,
    LeType!(1, NUM): Sized,
{
    #[inline(always)]
    pub fn get(this: &Self) -> &T {
        unsafe { this.inner.as_ref() }
    }

    #[inline(always)]
    pub unsafe fn get_mut_unchecked(this: &mut Self) -> &mut T {
        unsafe { this.inner.as_mut() }
    }

    #[inline(always)]
    pub fn has_same_pointee<const OTHER_NUM: usize>(&self, other: &Trc<T, OTHER_NUM, DEN>) -> bool
    where LeType!(1, OTHER_NUM): Sized,
        LeType!(OTHER_NUM, DEN): Sized,
    {
        self.inner.as_ptr() == other.inner.as_ptr()
    }

    #[inline(always)]
    pub fn join<const A: usize, const B: usize>(a: Trc<T, A, DEN>, b: Trc<T, B, DEN>) -> Self
    where
        EqType!(A + B, NUM): Sized,
        LeType!(1, A): Sized,
        LeType!(1, B): Sized,
        LeType!(A, DEN): Sized,
        LeType!(B, DEN): Sized,
        LeType!(A, NUM): Sized,
        LeType!(B, NUM): Sized,
    {
        assert!(a.has_same_pointee(&b));
        std::mem::forget(b);
        unsafe { Self::lift_from(a) }
    }

    #[inline(always)]
    pub fn split<const A: usize, const B: usize>(self) -> (Trc<T, A, DEN>, Trc<T, B, DEN>)
    where
        EqType!(A + B, NUM): Sized,
        LeType!(1, A): Sized,
        LeType!(1, B): Sized,
        LeType!(A, DEN): Sized,
        LeType!(B, DEN): Sized,
        LeType!(A, NUM): Sized,
        LeType!(B, NUM): Sized,
    {
        let inner = self.inner;
        std::mem::forget(self);
        (Trc::<T, A, DEN> { inner }, Trc::<T, B, DEN> { inner })
    }

    /// # Safety
    /// Free `(NUM - OLD_NUM) / DEN` ration before lifting.
    #[inline(always)]
    pub unsafe fn lift_from<const OLD_NUM: usize>(this: Trc<T, OLD_NUM, DEN>) -> Self
    where
        LeType!(OLD_NUM, NUM): Sized,
        LeType!(1, OLD_NUM): Sized,
    {
        let inner = this.inner;
        std::mem::forget(this);
        Self { inner }
    }
}

/// Impl for unique references.
impl<T, const N: usize> Trc<T, N, N>
where
    LeType!(1, N): Sized,
{
    pub fn new(val: T) -> Self {
        let ptr = Box::new(val);
        let ptr = Box::leak(ptr);
        let ptr = NonNull::from(ptr);

        Self { inner: ptr }
    }

    #[inline(always)]
    pub fn get_mut(this: &mut Self) -> &mut T {
        unsafe { this.inner.as_mut() }
    }

    #[inline(always)]
    pub fn into_inner(self) -> T {
        let val = unsafe { self.inner.as_ptr().read() };
        std::mem::forget(self);
        val
    }
}

impl<T, const NUM: usize, const DEN: usize> Drop for Trc<T, NUM, DEN> {
    fn drop(&mut self) {
        debug_assert_eq!(NUM, DEN);

        if NUM == DEN {
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        }
    }
}

impl<T, const NUM: usize, const DEN: usize> Deref for Trc<T, NUM, DEN>
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref() }
    }
}

impl<T, const N: usize> DerefMut for Trc<T, N, N>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut() }
    }
}
