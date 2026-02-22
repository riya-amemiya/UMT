/**
 * A node in the doubly linked list used by LRUCache.
 * @template K - The type of the key
 * @template V - The type of the value
 */
class LRUNode<K, V> {
  key: K;
  value: V;
  prev: LRUNode<K, V> | null = null;
  next: LRUNode<K, V> | null = null;

  constructor(key: K, value: V) {
    this.key = key;
    this.value = value;
  }
}

/**
 * A Least Recently Used (LRU) cache implementation
 * using a Map and a doubly linked list for O(1) get/set operations.
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
  private map = new Map<K, LRUNode<K, V>>();
  private head: LRUNode<K, V> | null = null;
  private tail: LRUNode<K, V> | null = null;

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
    const node = this.map.get(key);
    if (node === undefined) {
      return;
    }
    this.moveToHead(node);
    return node.value;
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
    const existing = this.map.get(key);
    if (existing !== undefined) {
      existing.value = value;
      this.moveToHead(existing);
      return;
    }

    const node = new LRUNode(key, value);
    this.map.set(key, node);
    this.addToHead(node);

    if (this.map.size > this.capacity) {
      this.evict();
    }
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
    const node = this.map.get(key);
    if (node === undefined) {
      return false;
    }
    this.removeNode(node);
    this.map.delete(key);
    return true;
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
    this.head = null;
    this.tail = null;
  }

  /**
   * Moves an existing node to the head (most recently used).
   * @param node - The node to move
   */
  private moveToHead(node: LRUNode<K, V>): void {
    if (node === this.head) {
      return;
    }
    this.removeNode(node);
    this.addToHead(node);
  }

  /**
   * Removes a node from the doubly linked list.
   * @param node - The node to remove
   */
  private removeNode(node: LRUNode<K, V>): void {
    if (node.prev === null) {
      this.head = node.next;
    } else {
      node.prev.next = node.next;
    }

    if (node.next === null) {
      this.tail = node.prev;
    } else {
      node.next.prev = node.prev;
    }

    node.prev = null;
    node.next = null;
  }

  /**
   * Adds a node to the head of the doubly linked list.
   * @param node - The node to add
   */
  private addToHead(node: LRUNode<K, V>): void {
    node.next = this.head;
    node.prev = null;

    if (this.head !== null) {
      this.head.prev = node;
    }

    this.head = node;

    this.tail ??= node;
  }

  /**
   * Evicts the least recently used entry (tail of the list).
   */
  private evict(): void {
    if (this.tail === null) {
      return;
    }
    const evicted = this.tail;
    this.removeNode(evicted);
    this.map.delete(evicted.key);
  }
}
