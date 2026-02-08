/**
 * Configuration options for TTLCache.
 */
export interface TTLCacheOptions {
  /** Default time-to-live in milliseconds for cache entries */
  defaultTTL: number;
  /** Optional maximum number of entries */
  maxSize?: number;
}

/**
 * A time-to-live (TTL) cache that automatically expires entries
 * after a configured duration.
 *
 * Uses lazy deletion: expired entries are only removed
 * when accessed via get() or has().
 *
 * ## Features
 * - **get(key)**: Retrieve a value (returns undefined if expired)
 * - **set(key, value, ttl?)**: Insert with optional per-entry TTL
 * - **has(key)**: Check if a non-expired key exists
 * - **delete(key)**: Remove a specific entry
 * - **clear()**: Remove all entries
 * - **size**: Get the number of entries (including expired)
 *
 * @example
 * ```typescript
 * const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
 * cache.set("a", 1);
 * cache.get("a"); // 1
 *
 * // After 5 seconds...
 * cache.get("a"); // undefined (expired)
 * ```
 *
 * @template K - The type of cache keys
 * @template V - The type of cache values
 */
export class TTLCache<K, V> {
  private defaultTTL: number;
  private maxSize: number | undefined;
  private map = new Map<K, { value: V; expiresAt: number }>();

  /**
   * Creates a new TTLCache instance.
   * @param options - Configuration options for the cache
   *
   * @example
   * ```typescript
   * const cache = new TTLCache<string, number>({
   *   defaultTTL: 60000,
   *   maxSize: 1000,
   * });
   * ```
   */
  constructor(options: TTLCacheOptions) {
    this.defaultTTL = options.defaultTTL;
    this.maxSize = options.maxSize;
  }

  /**
   * Returns the number of entries in the cache
   * (including potentially expired entries).
   * @returns The current number of entries
   *
   * @example
   * ```typescript
   * const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
   * cache.set("a", 1);
   * console.log(cache.size); // 1
   * ```
   */
  get size(): number {
    return this.map.size;
  }

  /**
   * Retrieves a value by key. Returns undefined if the key
   * does not exist or has expired.
   * @param key - The key to look up
   * @returns The value if found and not expired, or undefined
   *
   * @example
   * ```typescript
   * const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
   * cache.set("a", 1);
   * cache.get("a"); // 1
   * ```
   */
  get(key: K): V | undefined {
    const entry = this.map.get(key);
    if (entry === undefined) {
      return;
    }
    if (Date.now() >= entry.expiresAt) {
      this.map.delete(key);
      return;
    }
    return entry.value;
  }

  /**
   * Inserts or updates a key-value pair with an optional TTL override.
   * If maxSize is configured and the cache is full, the oldest entry
   * is removed.
   * @param key - The key to set
   * @param value - The value to cache
   * @param ttl - Optional TTL in milliseconds (overrides defaultTTL)
   *
   * @example
   * ```typescript
   * const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
   * cache.set("a", 1);
   * cache.set("b", 2, 10000); // custom 10s TTL
   * ```
   */
  set(key: K, value: V, ttl?: number): void {
    const effectiveTTL = ttl ?? this.defaultTTL;
    const expiresAt = Date.now() + effectiveTTL;

    if (this.map.has(key)) {
      this.map.set(key, { value, expiresAt });
      return;
    }

    if (this.maxSize !== undefined && this.map.size >= this.maxSize) {
      const firstKey = this.map.keys().next().value as K;
      this.map.delete(firstKey);
    }

    this.map.set(key, { value, expiresAt });
  }

  /**
   * Checks whether a non-expired key exists in the cache.
   * Removes the entry if it has expired.
   * @param key - The key to check
   * @returns True if the key exists and has not expired
   *
   * @example
   * ```typescript
   * const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
   * cache.set("a", 1);
   * cache.has("a"); // true
   * cache.has("b"); // false
   * ```
   */
  has(key: K): boolean {
    const entry = this.map.get(key);
    if (entry === undefined) {
      return false;
    }
    if (Date.now() >= entry.expiresAt) {
      this.map.delete(key);
      return false;
    }
    return true;
  }

  /**
   * Removes an entry from the cache by key.
   * @param key - The key to remove
   * @returns True if the entry was found and removed
   *
   * @example
   * ```typescript
   * const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
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
   * const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
   * cache.set("a", 1);
   * cache.clear();
   * console.log(cache.size); // 0
   * ```
   */
  clear(): void {
    this.map.clear();
  }
}
