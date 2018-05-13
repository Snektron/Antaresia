use std::collections::{HashMap as Map, HashSet as Set};
use std::collections::hash_map::{Iter as MapIter, Drain};
use std::collections::hash_set::Iter as SetIter;
use std::cmp::Eq;
use std::hash::Hash;
use std::iter::Iterator;

#[derive(Debug, Clone)]
pub struct MultiMap<K, V>
where K: Eq + Hash,
      V: Eq + Hash {
    inner: Map<K, Set<V>>
}

impl<K, V> MultiMap<K, V>
where K: Eq + Hash {
    pub fn new() -> Self {
        MultiMap {
            inner: Map::new()
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.inner.entry(key).or_insert(Vec::new()).push(value);
    }

    pub fn insert_all(&mut self, key: K, ref mut value: Vec<V>) {
        self.inner.entry(key).or_insert(Vec::new()).append(value);
    }

    pub fn remove_all(&mut self, key: &K) -> Option<Vec<V>> {
        self.inner.remove(key)
    }

    pub fn get_all(&self, key: &K) -> Option<&Vec<V>> {
        self.inner.get(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter::new(self.inner.iter())
    }

    pub fn iter_all<'a>(&'a self) -> MapIter<'a, K, Vec<V>> {
        self.inner.iter()
    }

    pub fn drain_all<'a>(&'a mut self) -> Drain<'a, K, Vec<V>> {
        self.inner.drain()
    }
}

pub struct Iter<'a, K, V>
where K: 'a,
      V: 'a {
    map_iter: MapIter<'a, K, Vec<V>>,
    current: Option<(&'a K, SetIter<'a, V>)>
}

impl<'a, K, V> Iter<'a, K, V> {
    fn new(mut map_iter: MapIter<'a, K, Vec<V>>) -> Self {
        match map_iter.next() {
            None => Iter {
                map_iter,
                current: None
            },
            Some((key, values)) => Iter {
                map_iter,
                current: Some((key, values.iter()))
            }
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((key, ref mut value_iter)) = self.current {
                if let Some(value) = value_iter.next() {
                    return Some((key, value));
                }
            } else {
                return None;
            }

            if let Some((key, values)) = self.map_iter.next() {
                self.current = Some((key, values.iter()));
            } else {
                self.current = None;
            }
        }
    }
}