/**
Lazily combines values from several iterables at corresponding positions
Stops as soon as any iterable is exhausted and then closes every source iterator.
@param {Iterable<unknown>[]} iterables - The source iterables
@returns {Generator<T, void, undefined>} A generator yielding one tuple per position
@example
const pairs = lazyZip([1, 2, 3], ["a", "b"]);
[...pairs]; // [[1, "a"], [2, "b"]]
*/
export function* lazyZip<T extends unknown[]>(
  ...iterables: { [K in keyof T]: Iterable<T[K]> }
): Generator<T, void, undefined> {
  if (iterables.length === 0) {
    return;
  }
  const iterators: Iterator<unknown>[] = iterables.map((iterable) =>
    iterable[Symbol.iterator](),
  );
  try {
    while (true) {
      const tuple: unknown[] = new Array(iterators.length);
      for (const [index, iterator] of iterators.entries()) {
        const step = iterator.next();
        if (step.done) {
          return;
        }
        tuple[index] = step.value;
      }
      yield tuple as T;
    }
  } finally {
    for (const iterator of iterators) {
      iterator.return?.();
    }
  }
}
