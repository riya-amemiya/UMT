/**
 * 共通しない要素をとりだす
 * @param  {unknown[]} array
 * @param  {unknown[]} ...arrays
 * @returns unknown[]
 * @example getArraysDiff([1, 2, 3], [2, 3, 4]); // [1, 4]
 */
export const getArraysDiff = <T>(array: T[], ...arrays: T[][]): T[] => {
  const allValues = new Set(array);
  const duplicates = new Set<T>();

  for (const array_ of arrays) {
    for (const value of array_) {
      if (allValues.has(value)) {
        duplicates.add(value);
      } else {
        allValues.add(value);
      }
    }
  }

  return [...allValues].filter((value) => !duplicates.has(value));
};
