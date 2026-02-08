/**
 * Options for the memoize function.
 */
export interface MemoizeOptions<K> {
  /** Maximum number of entries to store in the cache. */
  maxSize?: number;
  /** Custom function to generate cache keys from arguments. */
  resolver?: (...arguments_: unknown[]) => K;
}

/**
 * A memoized function with an exposed cache.
 */
export interface MemoizedFunction<A extends unknown[], R, K = unknown> {
  (...arguments_: A): R;
  /** The underlying cache Map. */
  cache: Map<K, R>;
}

/**
 * Creates a memoized version of the provided function. Results are
 * cached in a Map keyed by the first argument (or a custom resolver).
 *
 * @param function_ - The function to memoize.
 * @param options - Configuration for cache behavior.
 * @param options.maxSize - Maximum cache entries before evicting the oldest.
 * @param options.resolver - Custom key resolver function.
 * @returns The memoized function with an exposed cache property.
 *
 * @example
 * ```typescript
 * const memoized = memoize((n: number) => n * 2);
 * memoized(5); // 10 (computed)
 * memoized(5); // 10 (cached)
 * memoized.cache.size; // 1
 * ```
 */
export const memoize = <A extends unknown[], R, K = unknown>(
  function_: (...arguments_: A) => R,
  options: MemoizeOptions<K> = {},
): MemoizedFunction<A, R, K> => {
  const { maxSize, resolver } = options;
  const cache = new Map<K, R>();

  const memoized = (
    resolver
      ? function (this: unknown, ...arguments_: A): R {
          const key = resolver(...arguments_);
          const cached = cache.get(key);
          if (cached !== undefined) {
            return cached;
          }
          const result = function_.apply(this, arguments_);
          if (maxSize !== undefined && cache.size >= maxSize) {
            const firstKey = cache.keys().next().value as K;
            cache.delete(firstKey);
          }
          cache.set(key, result);
          return result;
        }
      : function (this: unknown, ...arguments_: A): R {
          const key = arguments_[0] as K;
          const cached = cache.get(key);
          if (cached !== undefined) {
            return cached;
          }
          const result = function_.apply(this, arguments_);
          if (maxSize !== undefined && cache.size >= maxSize) {
            const firstKey = cache.keys().next().value as K;
            cache.delete(firstKey);
          }
          cache.set(key, result);
          return result;
        }
  ) as MemoizedFunction<A, R, K>;

  memoized.cache = cache;

  return memoized;
};
