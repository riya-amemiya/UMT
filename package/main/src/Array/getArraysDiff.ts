/**
 * Extract elements that are not common between arrays
 * @param  {T[]} array The first array
 * @param  {T[]} arrays Additional arrays to compare
 * @returns {unknown[]} Array containing elements that appear only once across all arrays
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
