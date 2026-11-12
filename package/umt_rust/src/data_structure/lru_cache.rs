use std::collections::HashMap;
use std::hash::Hash;

const NIL: usize = usize::MAX;

struct LRUNode<K, V> {
    key: K,
    value: V,
    prev: usize,
    next: usize,
}

/// A Least Recently Used (LRU) cache implementation
/// using a hash map and an index-linked list for O(1) get/set operations.
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
/// - get: O(1)
/// - set: O(1)
/// - has: O(1)
/// - delete: O(1)
pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, usize>,
    nodes: Vec<LRUNode<K, V>>,
    head: usize,
    tail: usize,
}

impl<K: Eq + Hash + Clone, V> LRUCache<K, V> {
    /// Creates a new LRUCache instance.
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            nodes: Vec::new(),
            head: NIL,
            tail: NIL,
        }
    }

    /// Returns the number of entries in the cache.
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Retrieves a value by key and marks it as most recently used.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let index = *self.map.get(key)?;
        self.detach(index);
        self.attach_back(index);
        Some(&self.nodes[index].value)
    }

    /// Inserts or updates a key-value pair.
    pub fn set(&mut self, key: K, value: V) {
        if let Some(&index) = self.map.get(&key) {
            self.nodes[index].value = value;
            self.detach(index);
            self.attach_back(index);
            return;
        }
        if self.map.len() >= self.capacity {
            self.evict();
        }
        let index = self.nodes.len();
        self.nodes.push(LRUNode {
            key: key.clone(),
            value,
            prev: NIL,
            next: NIL,
        });
        self.map.insert(key, index);
        self.attach_back(index);
    }

    /// Checks whether a key exists in the cache.
    pub fn has(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Removes an entry from the cache by key.
    pub fn delete(&mut self, key: &K) -> bool {
        match self.map.remove(key) {
            Some(index) => {
                self.remove_node(index);
                true
            }
            None => false,
        }
    }

    /// Removes all entries from the cache.
    pub fn clear(&mut self) {
        self.map.clear();
        self.nodes.clear();
        self.head = NIL;
        self.tail = NIL;
    }

    /// Evicts the least recently used entry (first inserted).
    fn evict(&mut self) {
        if self.head != NIL {
            let node = self.remove_node(self.head);
            self.map.remove(&node.key);
        }
    }

    /// Unlinks the node at `index` from the recency list.
    fn detach(&mut self, index: usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;
        if prev == NIL {
            self.head = next;
        } else {
            self.nodes[prev].next = next;
        }
        if next == NIL {
            self.tail = prev;
        } else {
            self.nodes[next].prev = prev;
        }
    }

    /// Links the node at `index` as the most recently used entry.
    fn attach_back(&mut self, index: usize) {
        self.nodes[index].prev = self.tail;
        self.nodes[index].next = NIL;
        if self.tail == NIL {
            self.head = index;
        } else {
            self.nodes[self.tail].next = index;
        }
        self.tail = index;
    }

    /// Removes the node at `index` from storage, relocating the last node into
    /// its slot.
    fn remove_node(&mut self, index: usize) -> LRUNode<K, V> {
        self.detach(index);
        let node = self.nodes.swap_remove(index);
        if index < self.nodes.len() {
            let prev = self.nodes[index].prev;
            let next = self.nodes[index].next;
            if prev == NIL {
                self.head = index;
            } else {
                self.nodes[prev].next = index;
            }
            if next == NIL {
                self.tail = index;
            } else {
                self.nodes[next].prev = index;
            }
            if let Some(slot) = self.map.get_mut(&self.nodes[index].key) {
                *slot = index;
            }
        }
        node
    }
}
