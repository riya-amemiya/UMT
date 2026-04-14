/**
 * A Bloom filter — a space-efficient probabilistic data structure
 * that tests whether an element is a member of a set.
 *
 * ## Properties
 * - **No false negatives**: if `has()` returns `false`, the element is
 *   definitely not in the set.
 * - **Possible false positives**: if `has()` returns `true`, the element
 *   is probably in the set (small chance of collision).
 * - **No deletion**: once an element is added it cannot be removed
 *   (use `clear()` to reset entirely).
 *
 * ## How it works
 * Internally maintains a bit array of `bitSize` bits.
 * `add(item)` hashes the item `k` times and sets the corresponding bits.
 * `has(item)` hashes the item the same way and returns `true` only when
 * all corresponding bits are set.
 *
 * Hash positions are derived via the Kirsch-Mitzenmacher double-hashing
 * technique:
 * ```
 * g_i(x) = (h1(x) + i * h2(x)) mod m
 * ```
 * where `h1` is FNV-1a and `h2` is djb2.
 *
 * ## Features
 * - **add(item)**: Add a string to the filter
 * - **has(item)**: Test membership (may return false positives)
 * - **clear()**: Reset all bits
 * - **bitSize**: Number of bits in the internal array
 * - **numHashFunctions**: Number of hash functions used
 * - **estimatedFalsePositiveRate(n)**: Theoretical false-positive rate after
 *   inserting `n` items
 * - **BloomFilter.fromExpected(n, p)**: Factory that calculates optimal
 *   parameters for `n` expected items at false-positive rate `p`
 *
 * ## Time Complexity
 * - add:  O(k)  where k = number of hash functions
 * - has:  O(k)
 * - clear: O(m/8)  where m = bit array size
 *
 * @example
 * ```typescript
 * // Manual parameters
 * const filter = new BloomFilter({ size: 1000, hashCount: 7 });
 * filter.add("hello");
 * filter.has("hello"); // true
 * filter.has("world"); // false (probably)
 * ```
 *
 * @example
 * ```typescript
 * // Optimal parameters for expected load
 * const filter = BloomFilter.fromExpected(1000, 0.01);
 * filter.add("alice@example.com");
 * filter.has("alice@example.com"); // true
 * filter.has("bob@example.com");   // false (probably)
 * console.log(filter.estimatedFalsePositiveRate(1000)); // ~0.01
 * ```
 */
export class BloomFilter {
  private static readonly fnv1aOffsetBasis = 2_166_136_261;
  private static readonly fnv1aPrime = 16_777_619;
  private static readonly djb2Seed = 5381;
  private static readonly djb2Multiplier = 33;

  private readonly bits: Uint8Array;
  private readonly size: number;
  private readonly hashCount: number;

  /**
   * Creates a new BloomFilter with explicit parameters.
   *
   * @param options.size      - Number of bits in the internal array (default: 1000)
   * @param options.hashCount - Number of hash functions to apply (default: 7)
   *
   * @example
   * ```typescript
   * const filter = new BloomFilter({ size: 2048, hashCount: 5 });
   * ```
   */
  constructor({
    size = 1000,
    hashCount = 7,
  }: { size?: number; hashCount?: number } = {}) {
    this.size = size;
    this.hashCount = hashCount;
    this.bits = new Uint8Array(Math.ceil(size / 8));
  }

  /**
   * Factory method that calculates optimal `size` and `hashCount` for the
   * given expected number of insertions and desired false-positive rate.
   *
   * Formulas used:
   * - `m = ceil(-n * ln(p) / ln(2)^2)`
   * - `k = round((m / n) * ln(2))`
   *
   * @param expectedItems    - Expected number of items to be inserted
   * @param falsePositiveRate - Desired false-positive probability (0 < p < 1)
   * @returns A new BloomFilter configured for the given constraints
   *
   * @example
   * ```typescript
   * // 1 % false-positive rate for up to 10 000 items
   * const filter = BloomFilter.fromExpected(10_000, 0.01);
   * ```
   */
  static fromExpected(
    expectedItems: number,
    falsePositiveRate: number,
  ): BloomFilter {
    const size = BloomFilter.optimalSize(expectedItems, falsePositiveRate);
    const hashCount = BloomFilter.optimalHashCount(size, expectedItems);
    return new BloomFilter({ size, hashCount });
  }

  /**
   * Returns the number of bits in the internal array.
   *
   * @example
   * ```typescript
   * const filter = new BloomFilter({ size: 512 });
   * console.log(filter.bitSize); // 512
   * ```
   */
  get bitSize(): number {
    return this.size;
  }

  /**
   * Returns the number of hash functions used.
   *
   * @example
   * ```typescript
   * const filter = new BloomFilter({ hashCount: 5 });
   * console.log(filter.numHashFunctions); // 5
   * ```
   */
  get numHashFunctions(): number {
    return this.hashCount;
  }

  /**
   * Adds one or more string items to the Bloom filter.
   * Sets `k` bits per item derived from its hash values.
   *
   * @param items - One or more strings to add
   *
   * @example
   * ```typescript
   * const filter = new BloomFilter();
   * filter.add("hello");
   * filter.add("foo", "bar", "baz");
   * filter.has("foo"); // true
   * ```
   */
  add(...items: string[]): void {
    for (const item of items) {
      for (const index of this.hashIndices(item)) {
        this.setBit(index);
      }
    }
  }

  /**
   * Tests whether an item is probably in the filter.
   *
   * - Returns `false` → the item is **definitely not** in the filter.
   * - Returns `true`  → the item is **probably** in the filter
   *   (false positives are possible).
   *
   * @param item - The string to test
   * @returns `true` if the item is probably present, `false` if definitely absent
   *
   * @example
   * ```typescript
   * const filter = new BloomFilter();
   * filter.add("hello");
   * filter.has("hello"); // true
   * filter.has("world"); // false (probably)
   * ```
   */
  has(item: string): boolean {
    return this.hashIndices(item).every((index) => this.getBit(index));
  }

  /**
   * Resets all bits, effectively emptying the filter.
   *
   * @example
   * ```typescript
   * const filter = new BloomFilter();
   * filter.add("hello");
   * filter.clear();
   * filter.has("hello"); // false
   * ```
   */
  clear(): void {
    this.bits.fill(0);
  }

  /**
   * Returns the theoretical false-positive probability after `insertedCount`
   * items have been added.
   *
   * Formula: `(1 - e^(-k * n / m)) ^ k`
   *
   * @param insertedCount - Number of items already inserted
   * @returns Estimated false-positive probability in [0, 1]
   *
   * @example
   * ```typescript
   * const filter = BloomFilter.fromExpected(1000, 0.01);
   * console.log(filter.estimatedFalsePositiveRate(1000)); // ~0.01
   * console.log(filter.estimatedFalsePositiveRate(500));  // lower than 0.01
   * ```
   */
  estimatedFalsePositiveRate(insertedCount: number): number {
    const exponent = (-this.hashCount * insertedCount) / this.size;
    return (1 - Math.exp(exponent)) ** this.hashCount;
  }

  private getBit(bitIndex: number): boolean {
    const byteIndex = Math.floor(bitIndex / 8);
    const offset = bitIndex % 8;
    return ((this.bits[byteIndex] ?? 0) & (1 << offset)) !== 0;
  }

  private setBit(bitIndex: number): void {
    const byteIndex = Math.floor(bitIndex / 8);
    const offset = bitIndex % 8;
    this.bits[byteIndex] |= 1 << offset;
  }

  /**
   * g_i(x) = (h1(x) + i * h2(x)) mod m
   */
  private hashIndices(item: string): number[] {
    const h1 = this.fnv1a(item);
    const h2 = this.djb2(item);
    return Array.from(
      { length: this.hashCount },
      (_, index) => ((h1 + Math.imul(index, h2)) >>> 0) % this.size,
    );
  }

  private fnv1a(string_: string): number {
    let hash = BloomFilter.fnv1aOffsetBasis;
    for (const char of string_) {
      hash =
        Math.imul(hash ^ (char.codePointAt(0) ?? 0), BloomFilter.fnv1aPrime) >>>
        0;
    }
    return hash;
  }

  private djb2(string_: string): number {
    let hash = BloomFilter.djb2Seed;
    for (const char of string_) {
      hash =
        (Math.imul(hash, BloomFilter.djb2Multiplier) ^
          (char.codePointAt(0) ?? 0)) >>>
        0;
    }
    return hash;
  }

  /**
   * m = ceil(-n * ln(p) / ln(2)^2)
   */
  private static optimalSize(n: number, p: number): number {
    return Math.ceil((-n * Math.log(p)) / Math.LN2 ** 2);
  }

  /**
   * k = round((m / n) * ln(2))
   */
  private static optimalHashCount(m: number, n: number): number {
    return Math.max(1, Math.round((m / n) * Math.LN2));
  }
}
