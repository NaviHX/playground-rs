use crate::utils::ghost_cell::{GhostCell, GhostToken};
use crate::utils::trc::Trc;

type Full<T> = Trc<T, 2, 2>;
type Half<T> = Trc<T, 1, 2>;

type FullPtr<'brand, T> = Full<GhostCell<'brand, Node<'brand, T>>>;
type HalfPtr<'brand, T> = Half<GhostCell<'brand, Node<'brand, T>>>;

struct Node<'brand, T> {
    pub val: T,
    prev: Option<HalfPtr<'brand, T>>,
    next: Option<HalfPtr<'brand, T>>,
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

pub struct LinkedList<'brand, T> {
    head_tail: Option<(HalfPtr<'brand, T>, HalfPtr<'brand, T>)>,
}

impl<T> Drop for LinkedList<'_, T> {
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

impl<T> Default for LinkedList<'_, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'brand, T> LinkedList<'brand, T> {
    pub fn new() -> Self {
        Self { head_tail: None }
    }

    pub fn push_front(&mut self, val: T, token: &mut GhostToken<'brand>) {
        let full = FullPtr::new(GhostCell::new(Node::new(val)));
        let (a, b) = full.split::<1, 1>();

        self.head_tail = match self.head_tail.take() {
            Some((head, tail)) => {
                head.borrow_mut(token).prev = Some(b);
                a.borrow_mut(token).next = Some(head);
                Some((a, tail))
            }
            None => Some((a, b)),
        }
    }

    pub fn push_back(&mut self, val: T, token: &mut GhostToken<'brand>) {
        let full = FullPtr::new(GhostCell::new(Node::new(val)));
        let (a, b) = full.split::<1, 1>();

        self.head_tail = match self.head_tail.take() {
            Some((head, tail)) => {
                tail.borrow_mut(token).next = Some(b);
                a.borrow_mut(token).prev = Some(tail);
                Some((head, a))
            }
            None => Some((a, b)),
        }
    }

    pub fn pop_front(&mut self, token: &mut GhostToken<'brand>) -> Option<T> {
        let (head, tail) = self.head_tail.take()?;

        if head.has_same_pointee(&tail) {
            let full = FullPtr::join(head, tail);
            let cell = full.into_inner();
            let node = cell.into_inner();
            let val = node.val;
            return Some(val);
        }

        let next = head.borrow_mut(token).next.take().expect("No next node!");
        let other_head = next.borrow_mut(token).prev.take().expect("No tail node!");
        let full = FullPtr::join(other_head, head);
        self.head_tail = Some((next, tail));

        Some(full.into_inner().into_inner().val)
    }

    pub fn pop_back(&mut self, token: &mut GhostToken<'brand>) -> Option<T> {
        let (head, tail) = self.head_tail.take()?;

        if head.has_same_pointee(&tail) {
            let full = FullPtr::join(head, tail);
            let cell = full.into_inner();
            let node = cell.into_inner();
            let val = node.val;
            return Some(val);
        }

        let prev = tail.borrow_mut(token).prev.take().expect("No prev node!");
        let other_tail = prev.borrow_mut(token).next.take().expect("No tail node!");
        let full = FullPtr::join(other_tail, tail);
        self.head_tail = Some((head, prev));

        Some(full.into_inner().into_inner().val)
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
