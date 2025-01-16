use std::{cell::UnsafeCell, marker::PhantomData};

type InvariantLifetime<'a> = PhantomData<fn(&'a ()) -> &'a ()>;

pub struct GhostCell<'brand, T> {
    inner: UnsafeCell<T>,
    _marker: InvariantLifetime<'brand>,
}

impl<'brand, T> GhostCell<'brand, T> {
    pub fn new(val: T) -> Self {
        Self { inner: UnsafeCell::new(val), _marker: InvariantLifetime::default() }
    }

    pub fn borrow<'a>(&'a self, _token: &'a GhostToken<'brand>) -> &'a T {
        unsafe { &*self.inner.get() }
    }

    pub fn borrow_mut<'a>(&'a self, _token: &'a mut GhostToken<'brand>) -> &'a mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub unsafe fn borrow_unchecked(&self) -> &T {
        unsafe { &*self.inner.get() }
    }

    pub unsafe fn borrow_mut_unchecked<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

pub struct GhostToken<'brand> {
    _marker: InvariantLifetime<'brand>,
}

unsafe impl Send for GhostToken<'_> {}
unsafe impl Sync for GhostToken<'_> {}

#[allow(clippy::needless_lifetimes)]
impl<'brand> GhostToken<'brand> {
    fn new() -> Self {
        Self {
            _marker: InvariantLifetime::default(),
        }
    }

    pub fn scope<F, R>(f: F) -> R
    where
        for<'new_brand> F: FnOnce(GhostToken<'new_brand>) -> R,
    {
        let token = Self::new();
        f(token)
    }
}
