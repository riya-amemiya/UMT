/**
 * 共通しない要素をとりだす
 * @param  {T[]} array
 * @param  {T[]} arrays
 * @returns unknown[]
 * @example getArraysDiff([1, 2, 3], [2, 3, 4]); // [1, 4]
 */
export const getArraysDiff = <O, T extends unknown[] = unknown[]>(
  array: T,
  ...arrays: T[]
): O => {
  const allValues = new Set(array);
  const duplicates = new Set<T>();

  for (const array_ of arrays) {
    for (const value of array_) {
      if (allValues.has(value)) {
        duplicates.add(value as T);
      } else {
        allValues.add(value);
      }
    }
  }

  return [...allValues].filter(
    (value) => !duplicates.has(value as T),
  ) as unknown as O;
};
