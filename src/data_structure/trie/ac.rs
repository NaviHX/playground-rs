use std::ptr::NonNull;

use super::{TrieImpl, TrieWalk};

pub struct FailTo<S> {
    p: Option<NonNull<TrieImpl<Self, S>>>,
}

impl<S> TrieWalk<S> for FailTo<S> {
    fn root() -> Self {
        Self { p: None }
    }

    fn build(parent_node: &TrieImpl<Self, S>, c: char) -> Self {
        let mut node = parent_node;

        loop {
            if let Some(p) = node.next.get(&c) {
                let p = &**p;
                break Self {
                    p: NonNull::new((p as *const _) as *mut _),
                };
            }

            match node.walk_info.p {
                Some(fail_to) => {
                    node = unsafe { fail_to.as_ref() };
                }
                None => {
                    break Self {
                        p: NonNull::new((node as *const _) as *mut _)
                    };
                }
            }
        }
    }

    fn walk(mut node: &TrieImpl<Self, S>, c: char) -> Option<&TrieImpl<Self, S>> {
        loop {
            if let Some(p) = node.next.get(&c) {
                break Some(&**p);
            }

            match node.walk_info.p {
                Some(fail_to) => {
                    node = unsafe { fail_to.as_ref() };
                }
                None => break Some(node),
            }
        }
    }
}
