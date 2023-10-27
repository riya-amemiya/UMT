/**
 * 共通しない要素をとりだす
 * @param  {unknown[]} array
 * @param  {unknown[]} ...arrays
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
