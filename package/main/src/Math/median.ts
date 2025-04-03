import { quickSort } from "@/Array/quickSort";

/**
 * Calculate the median of an array of numbers
 * @param {number[]} array Array of numbers
 * @returns {number} The median value
 * @example median([1, 3, 3, 6, 7, 8, 9]); // 6
 */
export const median = (array: number[]): number => {
  const sortedArray: number[] = quickSort(array);
  const mid = Math.floor(sortedArray.length / 2);

  return sortedArray.length % 2 === 0
    ? (sortedArray[mid - 1] + sortedArray[mid]) / 2
    : sortedArray[mid];
};
