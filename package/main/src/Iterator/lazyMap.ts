/**
 * Lazily maps values from an iterable using a generator
 * @param {Iterable<T>} iterable - The source iterable
 * @param {(value: T, index: number) => U} function_ - The mapping function
 * @returns {Generator<U, void, undefined>} A generator yielding mapped values
 * @example
 * const doubled = lazyMap([1, 2, 3], (n) => n * 2);
 * [...doubled]; // [2, 4, 6]
 */
export function* lazyMap<T, U>(
  iterable: Iterable<T>,
  function_: (value: T, index: number) => U,
): Generator<U, void, undefined> {
  let index = 0;
  for (const value of iterable) {
    yield function_(value, index);
    index += 1;
  }
}
