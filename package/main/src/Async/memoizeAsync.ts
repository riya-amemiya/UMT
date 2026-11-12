/**
Creates a memoized version of an async function. Concurrent calls that share
a key reuse a single in-flight promise, and resolved values are cached in a
Map keyed by the first argument (or a custom resolver). Rejected calls are
not cached, so the next call with the same key invokes the function again.

@template A - Argument tuple type
@template R - Resolved value type
@template K - Cache key type
@param function_ - The async function to memoize
@param options - Configuration for cache behavior
@param options.maxSize - Maximum cache entries before evicting the oldest
@param options.resolver - Custom key resolver function
@returns The memoized async function with an exposed cache of resolved values
@example
```typescript
const fetchUser = memoizeAsync((id: number) => api.getUser(id));
const [a, b] = await Promise.all([fetchUser(1), fetchUser(1)]);
// api.getUser is invoked once; a === b
fetchUser.cache.size; // 1
```
*/
export const memoizeAsync = <A extends unknown[], R, K = unknown>(
  function_: (...arguments_: A) => Promise<R>,
  options: { maxSize?: number; resolver?: (...arguments_: A) => K } = {},
): ((...arguments_: A) => Promise<R>) & { cache: Map<K, R> } => {
  const { maxSize, resolver } = options;
  const cache = new Map<K, R>();
  const pending = new Map<K, Promise<R>>();

  const memoized = function (this: unknown, ...arguments_: A): Promise<R> {
    const key = resolver ? resolver(...arguments_) : (arguments_[0] as K);
    if (cache.has(key)) {
      return Promise.resolve(cache.get(key) as R);
    }
    const inFlight = pending.get(key);
    if (inFlight) {
      return inFlight;
    }
    const promise = (async () => function_.apply(this, arguments_))();
    pending.set(key, promise);
    const track = async (): Promise<void> => {
      try {
        const value = await promise;
        pending.delete(key);
        if (maxSize !== undefined && cache.size >= maxSize) {
          cache.delete(cache.keys().next().value as K);
        }
        cache.set(key, value);
      } catch {
        pending.delete(key);
      }
    };
    track();
    return promise;
  };

  return Object.assign(memoized, { cache });
};
