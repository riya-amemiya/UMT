/**
 * Lazily filters values from an iterable using a generator
 * @param {Iterable<T>} iterable - The source iterable
 * @param {(value: T, index: number) => boolean} predicate - The filter predicate
 * @returns {Generator<T, void, undefined>} A generator yielding filtered values
 * @example
 * const evens = lazyFilter([1, 2, 3, 4], (n) => n % 2 === 0);
 * [...evens]; // [2, 4]
 */
export function* lazyFilter<T>(
  iterable: Iterable<T>,
  predicate: (value: T, index: number) => boolean,
): Generator<T, void, undefined> {
  let index = 0;
  for (const value of iterable) {
    if (predicate(value, index)) {
      yield value;
    }
    index += 1;
  }
}
