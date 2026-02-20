import { quickSort } from "@/Array/quickSort";

/**
 * Calculate the nth percentile of values in an array
 * @param array - Array of numbers
 * @param percentile - Percentile value (0-100)
 * @returns The percentile value
 * @example
 * percentile([1, 2, 3, 4, 5], 50); // 3 (50th percentile - median)
 * percentile([1, 2, 3, 4, 5], 25); // 2 (25th percentile)
 * percentile([1, 2, 3, 4, 5], 75); // 4 (75th percentile)
 */
export const percentile = (array: number[], percentile: number): number => {
  if (array.length === 0) {
    return Number.NaN;
  }

  const sortedArray = quickSort([...array]);
  const index = (percentile / 100) * (sortedArray.length - 1);
  const lowerIndex = Math.floor(index);
  const upperIndex = Math.ceil(index);

  if (lowerIndex === upperIndex) {
    return sortedArray[lowerIndex];
  }

  const lowerValue = sortedArray[lowerIndex];
  const upperValue = sortedArray[upperIndex];
  const weight = index - lowerIndex;

  return lowerValue + (upperValue - lowerValue) * weight;
};
