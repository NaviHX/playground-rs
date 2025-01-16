#![feature(generic_const_exprs)]

use std::{collections::HashMap, hash::Hash};
use playground_rs::data_structure::tripod_list::{Full, MapPtr, TripodList};
use playground_rs::utils::ghost_cell::GhostToken;

struct LruCache<'brand, K, V> {
    queue: TripodList<'brand, (K, V)>,
    map: HashMap<K, MapPtr<'brand, (K, V)>>,

    size: usize,
    capacity: usize,
}

impl<'brand, K: Hash + Eq, V> LruCache<'brand, K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: TripodList::new(),
            map: HashMap::new(),
            size: 0,
            capacity,
        }
    }

    pub fn get<'a, Q>(&'a mut self, q: &'a Q, token: &'a mut GhostToken<'brand>) -> Option<&'a V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq + ToOwned<Owned = K>,
    {
        let mapped_one = self.map.remove(q)?;
        let mapped_full = unsafe { self.queue.remove(mapped_one, token) };
        let front_one = self.queue.link_front(mapped_full, token);
        self.map.insert(q.to_owned(), front_one);
        self.queue.front(token).map(|(_, v)| v)
    }

    pub fn set<Q>(&mut self, q: Q, val: V, token: &mut GhostToken<'brand>)
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq + ToOwned<Owned = K>,
    {
        let one = self.queue.push_front((q.to_owned(), val), token);
        if let Some(old) = self.map.insert(q.to_owned(), one) {
            let _ = unsafe { self.queue.remove(old, token) };
            return;
        }

        if self.size < self.capacity {
            self.size += 1;
            return;
        }

        let back = self
            .queue
            .pop_back(token)
            .expect("The cache is full, but it has nothing in its queue!");
        let k = &back.borrow(token).val.0;
        let mapped_back = self
            .map
            .remove(k.borrow())
            .expect("Cannot find the mapped back!");
        let _ = Full::join(back, mapped_back);
    }
}

impl<K, V> Drop for LruCache<'_, K, V> {
    fn drop(&mut self) {
        for (_, mapped_one) in self.map.drain() {
            std::mem::forget(mapped_one);
        }
    }
}

fn main() {
    GhostToken::scope(|mut token| {
        let mut cache = LruCache::new(3);
        cache.set(1, 1, &mut token);
        assert_eq!(*cache.get(&1, &mut token).unwrap(), 1);

        cache.set(2, 2, &mut token);
        assert_eq!(*cache.get(&2, &mut token).unwrap(), 2);

        cache.set(3, 3, &mut token);
        assert_eq!(*cache.get(&3, &mut token).unwrap(), 3);
        assert_eq!(*cache.get(&1, &mut token).unwrap(), 1);

        cache.set(4, 4, &mut token);
        assert_eq!(*cache.get(&4, &mut token).unwrap(), 4);
        assert_eq!(cache.get(&2, &mut token), None);
    })
}
