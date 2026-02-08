/**
 * Lazily takes the first n values from an iterable
 * @param {Iterable<T>} iterable - The source iterable
 * @param {number} n - The number of values to take
 * @returns {Generator<T, void, undefined>} A generator yielding at most n values
 * @example
 * const first3 = lazyTake([1, 2, 3, 4, 5], 3);
 * [...first3]; // [1, 2, 3]
 */
export function* lazyTake<T>(
  iterable: Iterable<T>,
  n: number,
): Generator<T, void, undefined> {
  let count = 0;
  for (const value of iterable) {
    if (count >= n) {
      return;
    }
    yield value;
    count += 1;
  }
}
