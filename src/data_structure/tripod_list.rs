use crate::utils::{
    ghost_cell::{GhostCell, GhostToken},
    trc::Trc,
};

pub type Full<T> = Trc<T, 3, 3>;
type Two<T> = Trc<T, 2, 3>;
type One<T> = Trc<T, 1, 3>;

type FullPtr<'brand, T> = Full<GhostCell<'brand, Node<'brand, T>>>;
type TwoPtr<'brand, T> = Two<GhostCell<'brand, Node<'brand, T>>>;
type OnePtr<'brand, T> = One<GhostCell<'brand, Node<'brand, T>>>;

pub type MapPtr<'brand, T> = OnePtr<'brand, T>;

pub struct Node<'brand, T> {
    prev: Option<OnePtr<'brand, T>>,
    next: Option<OnePtr<'brand, T>>,
    pub val: T,
}

impl<T> Node<'_, T> {
    fn new(val: T) -> Self {
        Node {
            val,
            prev: None,
            next: None,
        }
    }
}

pub struct TripodList<'brand, T> {
    head_tail: Option<(OnePtr<'brand, T>, OnePtr<'brand, T>)>,
}

impl<T> Drop for TripodList<'_, T> {
    fn drop(&mut self) {
        if let Some((mut head, tail)) = self.head_tail.take() {
            std::mem::forget(tail);

            loop {
                if let Some(prev) = unsafe {
                    Trc::get_mut_unchecked(&mut head)
                        .borrow_mut_unchecked()
                        .prev
                        .take()
                } {
                    std::mem::forget(prev);
                }

                let next = unsafe {
                    Trc::get_mut_unchecked(&mut head)
                        .borrow_mut_unchecked()
                        .next
                        .take()
                };

                unsafe {
                    FullPtr::lift_from(head);
                }

                if let Some(next) = next {
                    head = next;
                } else {
                    break;
                }
            }
        }
    }
}

impl<'brand, T> TripodList<'brand, T> {
    pub fn new() -> Self {
        Self { head_tail: None }
    }

    pub fn link_front(
        &mut self,
        p: FullPtr<'brand, T>,
        token: &mut GhostToken<'brand>,
    ) -> OnePtr<'brand, T> {
        let (a, b) = p.split::<1, 2>();
        let (b, c) = b.split::<1, 1>();

        self.head_tail = match self.head_tail.take() {
            Some((head, tail)) => {
                head.borrow_mut(token).prev = Some(b);
                a.borrow_mut(token).next = Some(head);
                Some((a, tail))
            }
            None => Some((a, b)),
        };

        c
    }

    pub fn link_back(
        &mut self,
        p: FullPtr<'brand, T>,
        token: &mut GhostToken<'brand>,
    ) -> OnePtr<'brand, T> {
        let (a, b) = p.split::<1, 2>();
        let (b, c) = b.split::<1, 1>();

        self.head_tail = match self.head_tail.take() {
            Some((head, tail)) => {
                tail.borrow_mut(token).next = Some(b);
                a.borrow_mut(token).prev = Some(tail);
                Some((head, a))
            }
            None => Some((a, b)),
        };

        c
    }

    pub fn push_back(&mut self, val: T, token: &mut GhostToken<'brand>) -> OnePtr<'brand, T> {
        let full = FullPtr::new(GhostCell::new(Node::new(val)));
        self.link_back(full, token)
    }

    pub fn push_front(&mut self, val: T, token: &mut GhostToken<'brand>) -> OnePtr<'brand, T> {
        let full = FullPtr::new(GhostCell::new(Node::new(val)));
        self.link_front(full, token)
    }

    pub fn pop_front(&mut self, token: &mut GhostToken<'brand>) -> Option<TwoPtr<'brand, T>> {
        let (head, tail) = self.head_tail.take()?;

        if head.has_same_pointee(&tail) {
            let two = TwoPtr::join(head, tail);
            return Some(two);
        }

        let next = head.borrow_mut(token).next.take().expect("No next!");
        let other_head = next.borrow_mut(token).prev.take().expect("No head!");
        let two = TwoPtr::join(head, other_head);
        self.head_tail = Some((next, tail));
        Some(two)
    }

    pub fn pop_back(&mut self, token: &mut GhostToken<'brand>) -> Option<TwoPtr<'brand, T>> {
        let (head, tail) = self.head_tail.take()?;

        if head.has_same_pointee(&tail) {
            let two = TwoPtr::join(head, tail);
            return Some(two);
        }

        let prev = tail.borrow_mut(token).prev.take().expect("No prev!");
        let other_tail = prev.borrow_mut(token).next.take().expect("No tail!");
        let two = TwoPtr::join(tail, other_tail);
        self.head_tail = Some((head, prev));
        Some(two)
    }

    /// # Safety
    /// `one` is linked in `self`.
    pub unsafe fn remove(
        &mut self,
        one: OnePtr<'brand, T>,
        token: &mut GhostToken<'brand>,
    ) -> FullPtr<'brand, T> {
        let (head, tail) = self
            .head_tail
            .take()
            .expect("Trying to remove, but the list is empty!");
        let res =
            match (one.has_same_pointee(&head), one.has_same_pointee(&tail)) {
                (true, true) => {
                    let two = Two::join(head, tail);
                    Full::join(two, one)
                }
                (true, false) => {
                    self.head_tail = Some((head, tail));
                    let front = self
                        .pop_front(token)
                        .expect("The ptr to be removed is the head, but the list is empty!");
                    Full::join(front, one)
                }
                (false, true) => {
                    self.head_tail = Some((head, tail));
                    let back = self
                        .pop_back(token)
                        .expect("The ptr to be removed is the tail, but the list is empty!");
                    Full::join(back, one)
                }
                (false, false) => {
                    self.head_tail = Some((head, tail));
                    let prev = one.borrow_mut(token).prev.take().expect(
                        "The ptr to be removed is linked in the list, but it has not prev!",
                    );
                    let next = one.borrow_mut(token).next.take().expect(
                        "The ptr to be removed is linked in the list, but it has not next!",
                    );
                    let a =
                        prev.borrow_mut(token).next.take().expect(
                            "The ptr to be removed is not linked as next for the prev node",
                        );
                    let b =
                        next.borrow_mut(token).prev.take().expect(
                            "The ptr to be removed is not linked as prev for the next node",
                        );

                    // Link `prev` and `next`.
                    // # Safety
                    // Temporary lifting of `prev` is safe because we `forget` the new one third
                    // ownership after linking.
                    unsafe {
                        let prev = Two::lift_from(prev);
                        let (prev, other_prev) = prev.split::<1, 1>();
                        next.borrow_mut(token).prev = Some(other_prev);
                        prev.borrow_mut(token).next = Some(next);
                        std::mem::forget(prev);
                    }

                    let two = Two::join(a, b);
                    Full::join(two, one)
                }
            };

        res
    }

    pub fn front<'a>(&'a self, token: &'a GhostToken<'brand>) -> Option<&'a T> {
        self.head_tail
            .as_ref()
            .map(|(head, _)| &head.borrow(token).val)
    }

    pub fn front_mut<'a>(&'a mut self, token: &'a mut GhostToken<'brand>) -> Option<&'a mut T> {
        self.head_tail
            .as_ref()
            .map(|(head, _)| &mut head.borrow_mut(token).val)
    }

    pub fn back<'a>(&'a self, token: &'a GhostToken<'brand>) -> Option<&'a T> {
        self.head_tail
            .as_ref()
            .map(|(_, tail)| &tail.borrow(token).val)
    }

    pub fn back_mut<'a>(&'a mut self, token: &'a mut GhostToken<'brand>) -> Option<&'a mut T> {
        self.head_tail
            .as_ref()
            .map(|(_, tail)| &mut tail.borrow_mut(token).val)
    }
}

impl<T> Default for TripodList<'_, T> {
    fn default() -> Self {
        Self::new()
    }
}
