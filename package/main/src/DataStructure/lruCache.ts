/**
 * A Least Recently Used (LRU) cache implementation
 * using a Map for O(1) get/set operations.
 *
 * When the cache exceeds its capacity, the least recently used
 * entry is evicted.
 *
 * ## Features
 * - **get(key)**: Retrieve a value (moves it to most recently used)
 * - **set(key, value)**: Insert or update a value
 * - **has(key)**: Check if a key exists
 * - **delete(key)**: Remove a specific entry
 * - **clear()**: Remove all entries
 * - **size**: Get the number of entries
 *
 * ## Time Complexity
 * - get: O(1)
 * - set: O(1)
 * - has: O(1)
 * - delete: O(1)
 *
 * @example
 * ```typescript
 * const cache = new LRUCache<string, number>(3);
 * cache.set("a", 1);
 * cache.set("b", 2);
 * cache.set("c", 3);
 *
 * cache.get("a"); // 1 (moves "a" to most recently used)
 * cache.set("d", 4); // evicts "b" (least recently used)
 *
 * cache.has("b"); // false
 * cache.get("a"); // 1
 * ```
 *
 * @template K - The type of cache keys
 * @template V - The type of cache values
 */
export class LRUCache<K, V> {
  private capacity: number;
  private map = new Map<K, V>();

  /**
   * Creates a new LRUCache instance.
   * @param capacity - The maximum number of entries the cache can hold
   *
   * @example
   * ```typescript
   * const cache = new LRUCache<string, number>(100);
   * ```
   */
  constructor(capacity: number) {
    this.capacity = capacity;
  }

  /**
   * Returns the number of entries in the cache.
   * @returns The current number of cached entries
   *
   * @example
   * ```typescript
   * const cache = new LRUCache<string, number>(10);
   * cache.set("a", 1);
   * console.log(cache.size); // 1
   * ```
   */
  get size(): number {
    return this.map.size;
  }

  /**
   * Retrieves a value by key and marks it as most recently used.
   * @param key - The key to look up
   * @returns The value if found, or undefined if not in cache
   *
   * @example
   * ```typescript
   * const cache = new LRUCache<string, number>(10);
   * cache.set("a", 1);
   * cache.get("a"); // 1
   * cache.get("b"); // undefined
   * ```
   */
  get(key: K): V | undefined {
    const value = this.map.get(key);
    if (value === undefined && !this.map.has(key)) {
      return undefined;
    }
    // Refresh key: delete and re-insert
    this.map.delete(key);
    this.map.set(key, value as V);
    return value;
  }

  /**
   * Inserts or updates a key-value pair.
   * If the cache is at capacity, the least recently used entry is evicted.
   * @param key - The key to set
   * @param value - The value to associate with the key
   *
   * @example
   * ```typescript
   * const cache = new LRUCache<string, number>(2);
   * cache.set("a", 1);
   * cache.set("b", 2);
   * cache.set("c", 3); // evicts "a"
   * ```
   */
  set(key: K, value: V): void {
    if (this.map.has(key)) {
      this.map.delete(key);
    } else if (this.map.size >= this.capacity) {
      this.evict();
    }
    this.map.set(key, value);
  }

  /**
   * Checks whether a key exists in the cache.
   * Does not affect the recently-used order.
   * @param key - The key to check
   * @returns True if the key exists in the cache
   *
   * @example
   * ```typescript
   * const cache = new LRUCache<string, number>(10);
   * cache.set("a", 1);
   * cache.has("a"); // true
   * cache.has("b"); // false
   * ```
   */
  has(key: K): boolean {
    return this.map.has(key);
  }

  /**
   * Removes an entry from the cache by key.
   * @param key - The key to remove
   * @returns True if the entry was found and removed
   *
   * @example
   * ```typescript
   * const cache = new LRUCache<string, number>(10);
   * cache.set("a", 1);
   * cache.delete("a"); // true
   * cache.delete("b"); // false
   * ```
   */
  delete(key: K): boolean {
    return this.map.delete(key);
  }

  /**
   * Removes all entries from the cache.
   *
   * @example
   * ```typescript
   * const cache = new LRUCache<string, number>(10);
   * cache.set("a", 1);
   * cache.set("b", 2);
   * cache.clear();
   * console.log(cache.size); // 0
   * ```
   */
  clear(): void {
    this.map.clear();
  }

  /**
   * Evicts the least recently used entry (first inserted).
   */
  private evict(): void {
    const iterator = this.map.keys().next();
    if (!iterator.done) {
      this.map.delete(iterator.value);
    }
  }
}
