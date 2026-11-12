/**
Lazily groups values from an iterable into arrays of the given size
The last chunk holds the remaining values and may be shorter than size.
@param {Iterable<T>} iterable - The source iterable
@param {number} size - The number of values in each chunk
@returns {Generator<T[], void, undefined>} A generator yielding chunks
@example
const chunks = lazyChunk([1, 2, 3, 4, 5], 2);
[...chunks]; // [[1, 2], [3, 4], [5]]
*/
export function* lazyChunk<T>(
  iterable: Iterable<T>,
  size: number,
): Generator<T[], void, undefined> {
  const buffer: T[] = [];
  for (const value of iterable) {
    buffer.push(value);
    if (buffer.length === size) {
      yield buffer.splice(0);
    }
  }
  if (buffer.length > 0) {
    yield buffer;
  }
}
