use std::mem::swap;

pub struct BinaryHeap<T> {
    data: Vec<T>
}

impl<T> BinaryHeap<T> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Default for BinaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> BinaryHeap<T> {
    #[inline]
    fn parent(id: usize) -> usize {
        (id - 1) >> 1
    }

    #[inline]
    fn left_son(id: usize) -> usize {
        (id << 1) + 1
    }

    #[inline]
    fn right_son(id: usize) -> usize {
        (id << 1) + 2
    }

    pub fn push(&mut self, elem: T) {
        let mut pos = self.data.len();
        self.data.push(elem);

        while pos != 0 && self.data[pos] > self.data[Self::parent(pos)] {
            self.data.swap(pos, Self::parent(pos));
            pos = Self::parent(pos);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut item| {
            if !self.data.is_empty() {
                swap(&mut item, &mut self.data[0]);
                let mut pos = 0;
                let end = self.data.len();
                while Self::left_son(pos) < end {
                    let (left, right) = (Self::left_son(pos), Self::right_son(pos));

                    let mut t = if item > self.data[left] { pos } else { left };
                    if right < end {
                        t = if self.data[t] > self.data[right] { t } else { right };
                    }

                    if t == pos { break; }
                    self.data.swap(t, pos);
                    pos = t;
                }
            }

            item
        })
    }

    pub fn top(&self) -> Option<&T> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::BinaryHeap;

    #[test]
    fn push_and_pop() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        assert_eq!(heap.pop(), Some(1));
    }

    #[test]
    fn push_1_to_100() {
        let mut heap = BinaryHeap::new();
        for i in 1..=100 {
            heap.push(i);
        }

        assert_eq!(heap.pop(), Some(100));
    }
}
