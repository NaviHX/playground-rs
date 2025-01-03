use std::{hash::Hash, task::Waker};

pub trait Tag: Sized {
    type Tag;

    fn tag(&self) -> Self::Tag;

    fn tagged(self) -> Tagged<Self> {
        Tagged::new(self)
    }
}

pub struct Tagged<T: Tag> {
    tag: T::Tag,
    wrapped: T,
}

impl<T: Tag> Tagged<T> {
    pub fn new(data: T) -> Self {
        Self {
            tag: data.tag(),
            wrapped: data,
        }
    }

    pub fn tag(&self) -> &T::Tag {
        &self.tag
    }

    pub fn get(&self) -> &T {
        &self.wrapped
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.wrapped
    }
}

impl<T: Tag> Hash for Tagged<T>
where
    T::Tag: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}

impl<T: Tag> PartialEq for Tagged<T>
where
    T::Tag: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl<T: Tag> PartialOrd for Tagged<T>
where
    T::Tag: PartialOrd + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tag.partial_cmp(&other.tag)
    }
}

impl<T: Tag> Ord for Tagged<T>
where
    T::Tag: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tag.cmp(&other.tag)
    }
}

impl<T: Tag> Eq for Tagged<T> where T::Tag: Eq {}

impl Tag for Waker {
    type Tag = (*const (), *const ());

    fn tag(&self) -> Self::Tag {
        let (data, vtable) = (self.data(), self.vtable());
        let vtable_ptr = vtable as *const _ as *const ();

        (data, vtable_ptr)
    }

    fn tagged(self) -> Tagged<Self> {
        Tagged::new(self)
    }
}
