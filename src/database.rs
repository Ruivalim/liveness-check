use std::collections::HashMap;
use std::hash::Hash;

pub struct DIM<K: Eq + Hash, T> {
    storage: HashMap<K, T>,
}

impl<K: Eq + Hash, T> DIM<K, T> {
    pub fn new() -> Self {
        DIM {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: K, value: T) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&T> {
        self.storage.get(key)
    }

    pub fn delete(&mut self, key: &K) -> bool {
        self.storage.remove(key).is_some()
    }
}
