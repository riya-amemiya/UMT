/**
 * Randomly selects a specified number of elements from an array
 * @param array Source array
 * @param count Number of elements to select
 * @param allowDuplicates Whether to allow duplicate selections (default: false)
 * @returns Array of randomly selected elements
 * @example randomSelect([1, 2, 3, 4, 5], 2); // [3, 1]
 */
export const randomSelect = <T>(
  array: T[],
  count: number,
  allowDuplicates = false,
): T[] => {
  const result: T[] = [];
  const usedIndices = new Set<number>();

  while (
    result.length < count &&
    (allowDuplicates || result.length < array.length)
  ) {
    const randomIndex = Math.floor(Math.random() * array.length);
    if (allowDuplicates || !usedIndices.has(randomIndex)) {
      usedIndices.add(randomIndex);
      result.push(array[randomIndex]);
    }
  }

  return result;
};
