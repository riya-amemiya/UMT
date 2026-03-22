use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// A time-to-live (TTL) cache that automatically expires entries
/// after a configured duration.
///
/// Uses lazy deletion: expired entries are only removed
/// when accessed via `get()` or `has()`.
///
/// ## Features
/// - **get(key)**: Retrieve a value (returns `None` if expired)
/// - **set(key, value, ttl)**: Insert with optional per-entry TTL
/// - **has(key)**: Check if a non-expired key exists
/// - **delete(key)**: Remove a specific entry
/// - **clear()**: Remove all entries
/// - **size()**: Get the number of entries (including expired)
pub struct TTLCache<K, V> {
    default_ttl_ms: u64,
    max_size: Option<usize>,
    map: HashMap<K, TTLEntry<V>>,
    insertion_order: VecDeque<K>,
    now_ms_fn: Box<dyn Fn() -> u64>,
}

struct TTLEntry<V> {
    value: V,
    expires_at_ms: u64,
}

/// Returns current time in milliseconds since UNIX epoch.
fn system_now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before UNIX epoch")
        .as_millis() as u64
}

impl<K: Eq + Hash + Clone, V> TTLCache<K, V> {
    /// Creates a new TTLCache instance.
    ///
    /// # Arguments
    /// * `default_ttl_ms` - Default time-to-live in milliseconds for cache entries
    /// * `max_size` - Optional maximum number of entries
    pub fn new(default_ttl_ms: u64, max_size: Option<usize>) -> Self {
        TTLCache {
            default_ttl_ms,
            max_size,
            map: HashMap::new(),
            insertion_order: VecDeque::new(),
            now_ms_fn: Box::new(system_now_ms),
        }
    }

    /// Creates a new TTLCache with a custom clock function (for testing).
    #[doc(hidden)]
    pub fn with_clock(
        default_ttl_ms: u64,
        max_size: Option<usize>,
        now_ms_fn: Box<dyn Fn() -> u64>,
    ) -> Self {
        TTLCache {
            default_ttl_ms,
            max_size,
            map: HashMap::new(),
            insertion_order: VecDeque::new(),
            now_ms_fn,
        }
    }

    /// Returns the number of entries in the cache
    /// (including potentially expired entries).
    pub fn size(&self) -> usize {
        self.map.len()
    }

    #[inline]
    fn now_ms(&self) -> u64 {
        (self.now_ms_fn)()
    }

    /// Retrieves a value by key. Returns `None` if the key
    /// does not exist or has expired.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let now = self.now_ms();
        let expired = self
            .map
            .get(key)
            .is_some_and(|entry| now >= entry.expires_at_ms);

        if expired {
            self.map.remove(key);
            if let Some(pos) = self.insertion_order.iter().position(|k| k == key) {
                self.insertion_order.remove(pos);
            }
            return None;
        }

        self.map.get(key).map(|e| &e.value)
    }

    /// Inserts or updates a key-value pair with an optional TTL override.
    /// If `max_size` is configured and the cache is full, the oldest entry
    /// is removed.
    ///
    /// # Arguments
    /// * `key` - The key to set
    /// * `value` - The value to cache
    /// * `ttl_ms` - Optional TTL in milliseconds (overrides default_ttl_ms)
    pub fn set(&mut self, key: K, value: V, ttl_ms: Option<u64>) {
        let effective_ttl = ttl_ms.unwrap_or(self.default_ttl_ms);
        let expires_at_ms = self.now_ms() + effective_ttl;

        if let std::collections::hash_map::Entry::Occupied(mut e) = self.map.entry(key.clone()) {
            e.insert(TTLEntry {
                value,
                expires_at_ms,
            });
            return;
        }

        if let Some(max) = self.max_size
            && self.map.len() >= max
            && let Some(oldest_key) = self.insertion_order.pop_front()
        {
            self.map.remove(&oldest_key);
        }

        self.insertion_order.push_back(key.clone());
        self.map.insert(
            key,
            TTLEntry {
                value,
                expires_at_ms,
            },
        );
    }

    /// Checks whether a non-expired key exists in the cache.
    /// Removes the entry if it has expired.
    pub fn has(&mut self, key: &K) -> bool {
        let now = self.now_ms();
        let expired = self
            .map
            .get(key)
            .is_some_and(|entry| now >= entry.expires_at_ms);

        if expired {
            self.map.remove(key);
            if let Some(pos) = self.insertion_order.iter().position(|k| k == key) {
                self.insertion_order.remove(pos);
            }
            return false;
        }

        self.map.contains_key(key)
    }

    /// Removes an entry from the cache by key.
    /// Returns `true` if the entry was found and removed.
    pub fn delete(&mut self, key: &K) -> bool {
        if self.map.remove(key).is_some() {
            if let Some(pos) = self.insertion_order.iter().position(|k| k == key) {
                self.insertion_order.remove(pos);
            }
            true
        } else {
            false
        }
    }

    /// Removes all entries from the cache.
    pub fn clear(&mut self) {
        self.map.clear();
        self.insertion_order.clear();
    }
}
