/**
 * 共通しない要素をとりだす
 * @param  {unknown[]} array
 * @param  {unknown[]} ...arrays
 */
export const getArraysDiff = <T>(array: T[], ...arrays: T[][]): T[] => {
  const allValues = new Set(array);
  const duplicates = new Set<T>();

  for (const arr of arrays) {
    for (const val of arr) {
      if (allValues.has(val)) {
        duplicates.add(val);
      } else {
        allValues.add(val);
      }
    }
  }

  return Array.from(allValues).filter((val) => !duplicates.has(val));
};
