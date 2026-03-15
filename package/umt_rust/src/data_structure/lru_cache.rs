use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// A Least Recently Used (LRU) cache implementation
///
/// When the cache exceeds its capacity, the least recently used
/// entry is evicted.
///
/// ## Features
/// - **get(key)**: Retrieve a value (moves it to most recently used)
/// - **set(key, value)**: Insert or update a value
/// - **has(key)**: Check if a key exists
/// - **delete(key)**: Remove a specific entry
/// - **clear()**: Remove all entries
/// - **size**: Get the number of entries
///
/// ## Time Complexity
/// - get: O(N) where N is cache capacity, since we use VecDeque. But capacity is typically small.
/// - set: O(N)
/// - has: O(1)
/// - delete: O(N)
pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Eq + Hash + Clone, V> LRUCache<K, V> {
    /// Creates a new LRUCache instance.
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    /// Returns the number of entries in the cache.
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Retrieves a value by key and marks it as most recently used.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move to most recently used (back of the queue)
            if let Some(pos) = self.order.iter().position(|k| k == key) {
                let k = self.order.remove(pos).unwrap();
                self.order.push_back(k);
            }
            self.map.get(key)
        } else {
            None
        }
    }

    /// Inserts or updates a key-value pair.
    pub fn set(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            if let Some(pos) = self.order.iter().position(|k| k == &key) {
                self.order.remove(pos);
            }
            self.order.push_back(key.clone());
            self.map.insert(key, value);
        } else {
            if self.map.len() >= self.capacity {
                self.evict();
            }
            self.order.push_back(key.clone());
            self.map.insert(key, value);
        }
    }

    /// Checks whether a key exists in the cache.
    pub fn has(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Removes an entry from the cache by key.
    pub fn delete(&mut self, key: &K) -> bool {
        if self.map.remove(key).is_some() {
            if let Some(pos) = self.order.iter().position(|k| k == key) {
                self.order.remove(pos);
            }
            true
        } else {
            false
        }
    }

    /// Removes all entries from the cache.
    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }

    /// Evicts the least recently used entry (first inserted).
    fn evict(&mut self) {
        if let Some(key) = self.order.pop_front() {
            self.map.remove(&key);
        }
    }
}
