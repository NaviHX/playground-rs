mod ac;
pub type ACAutomata<S> = TrieImpl<ac::FailTo<S>, S>;

use std::collections::HashMap;

pub trait TrieWalk<S>: Sized {
    fn root() -> Self;
    fn build(parent_node: &TrieImpl<Self, S>, c: char) -> Self;
    fn walk(node: &TrieImpl<Self, S>, c: char) -> Option<&TrieImpl<Self, S>>;
}

pub struct TrieImpl<T, S> {
    pub next: HashMap<char, Box<TrieImpl<T, S>>>,
    pub walk_info: T,
    pub attached_info: Option<S>,
}

impl<T, S> TrieImpl<T, S> {
    fn new(attached_info: T) -> Self {
        Self {
            next: HashMap::new(),
            walk_info: attached_info,
            attached_info: None,
        }
    }

    fn boxed(attached_info: T) -> Box<Self> {
        Box::new(Self::new(attached_info))
    }
}

impl<T: TrieWalk<S>, S> TrieImpl<T, S> {
    pub fn insert(&mut self, s: impl IntoIterator<Item = char>, info: S) {
        let it = s.into_iter();
        let mut cur = self;

        for c in it {
            #[allow(clippy::map_entry)]
            if !cur.next.contains_key(&c) {
                cur.next.insert(c, Self::boxed(T::build(cur, c)));
            }

            cur = cur.next.get_mut(&c).unwrap();
        }

        cur.attached_info = Some(info)
    }

    pub fn walk(&self, s: impl IntoIterator<Item = char>) -> impl Iterator<Item = &Self> {
        TrieWalker::Continue {
            ptr: self,
            it: s.into_iter(),
        }
    }

    pub fn new_root() -> Self {
        Self::new(T::root())
    }

    pub fn new_boxed_root() -> Box<Self> {
        Box::new(Self::new_root())
    }
}

pub enum TrieWalker<'a, T, S, I> {
    Continue { ptr: &'a TrieImpl<T, S>, it: I },
    End,
}

impl<'a, T: TrieWalk<S>, S, I: Iterator<Item = char>> TrieWalker<'a, T, S, I> {
    fn and_then(
        &mut self,
        f: impl FnOnce(&'a TrieImpl<T, S>, char) -> Option<&'a TrieImpl<T, S>>,
    ) -> Option<&'a TrieImpl<T, S>> {
        match self {
            TrieWalker::Continue { ptr, it } => {
                if let Some(c) = it.next() {
                    let res = f(ptr, c);

                    match res {
                        Some(new_ptr) => {
                            *ptr = new_ptr;
                            Some(new_ptr)
                        }
                        None => {
                            *self = Self::End;
                            None
                        }
                    }
                } else {
                    *self = Self::End;
                    None
                }
            }
            TrieWalker::End => None,
        }
    }
}

impl<'a, T: TrieWalk<S>, S, I: Iterator<Item = char>> Iterator for TrieWalker<'a, T, S, I> {
    type Item = &'a TrieImpl<T, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.and_then(T::walk)
    }
}

impl<S> TrieWalk<S> for () {
    fn root() -> Self {}
    fn build(_parent_node: &TrieImpl<Self, S>, _c: char) -> Self {}

    fn walk(node: &TrieImpl<Self, S>, c: char) -> Option<&TrieImpl<Self, S>> {
        node.next.get(&c).map(|n| &**n)
    }
}

pub type Trie<S> = TrieImpl<(), S>;
