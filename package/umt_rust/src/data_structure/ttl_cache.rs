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
///
/// ## Time Complexity
/// - get: O(1)
/// - set: O(1) amortized
/// - has: O(1)
/// - delete: O(1)
pub struct TTLCache<K, V> {
    default_ttl_ms: u64,
    max_size: Option<usize>,
    map: HashMap<K, TTLEntry<V>>,
    insertion_order: VecDeque<(K, u64)>,
    next_sequence: u64,
    now_ms_fn: Box<dyn Fn() -> u64>,
}

struct TTLEntry<V> {
    value: V,
    expires_at_ms: u64,
    sequence: u64,
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
        Self::with_clock(default_ttl_ms, max_size, Box::new(system_now_ms))
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
            next_sequence: 0,
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

    /// Removes the entry for `key` when it has expired and reports whether it did.
    fn remove_if_expired(&mut self, key: &K) -> bool {
        let now = self.now_ms();
        let expired = self
            .map
            .get(key)
            .is_some_and(|entry| now >= entry.expires_at_ms);
        if expired {
            self.map.remove(key);
        }
        expired
    }

    /// Removes the oldest inserted entry that is still present.
    fn evict_oldest(&mut self) {
        while let Some((key, sequence)) = self.insertion_order.pop_front() {
            let is_live = self
                .map
                .get(&key)
                .is_some_and(|entry| entry.sequence == sequence);
            if is_live {
                self.map.remove(&key);
                return;
            }
        }
    }

    /// Records the insertion of `key`, dropping stale records once they
    /// outnumber the live entries.
    fn record_insertion(&mut self, key: K, sequence: u64) {
        if self.insertion_order.len() > 2 * self.map.len() + 16 {
            let map = &self.map;
            self.insertion_order.retain(|(recorded_key, recorded)| {
                map.get(recorded_key)
                    .is_some_and(|entry| entry.sequence == *recorded)
            });
        }
        self.insertion_order.push_back((key, sequence));
    }

    /// Retrieves a value by key. Returns `None` if the key
    /// does not exist or has expired.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.remove_if_expired(key) {
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

        if let Some(entry) = self.map.get_mut(&key) {
            entry.value = value;
            entry.expires_at_ms = expires_at_ms;
            return;
        }

        let sequence = self.next_sequence;
        self.next_sequence += 1;

        if let Some(max) = self.max_size {
            if self.map.len() >= max {
                self.evict_oldest();
            }
            self.record_insertion(key.clone(), sequence);
        }

        self.map.insert(
            key,
            TTLEntry {
                value,
                expires_at_ms,
                sequence,
            },
        );
    }

    /// Checks whether a non-expired key exists in the cache.
    /// Removes the entry if it has expired.
    pub fn has(&mut self, key: &K) -> bool {
        if self.remove_if_expired(key) {
            return false;
        }

        self.map.contains_key(key)
    }

    /// Removes an entry from the cache by key.
    /// Returns `true` if the entry was found and removed.
    pub fn delete(&mut self, key: &K) -> bool {
        self.map.remove(key).is_some()
    }

    /// Removes all entries from the cache.
    pub fn clear(&mut self) {
        self.map.clear();
        self.insertion_order.clear();
    }
}
