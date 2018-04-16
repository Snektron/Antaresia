use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::borrow::Borrow;

pub struct ScopedMap<K, V> {
    maps: Vec<HashMap<K, V>>
}

impl<K, V> ScopedMap<K, V>
where K: Hash + Eq {
    pub fn new() -> Self {
        ScopedMap {
            maps: Vec::new()
        }
    }

    pub fn enter(&mut self) {
        self.maps.push(HashMap::new());
    }

    pub fn exit(&mut self) {
        self.maps.pop();
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where Q: Borrow<K> {
        let key = key.borrow();

        for map in self.maps.iter() {
            if let Some(v) = map.get(key) {
                return Some(v);
            }
        }

        None
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        let top = self.maps.last_mut().unwrap();
        if top.contains_key(&key) {
            return false;
        }

        top.insert(key, value);
        true
    }
}