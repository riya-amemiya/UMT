import { compareFunctionDefault } from "./compareFunctionDefault";
import { insertionSortRange } from "./sortingHelpers/insertionSortRange";

import type { CompareFunction } from "$/array/compareFunction";

/**
 * Sort an array using insertion sort algorithm
 *
 * @param array Array to sort
 * @param compareFunction Function to compare two elements
 * @param start Starting index for sorting (inclusive)
 * @param end Ending index for sorting (inclusive)
 * @returns Sorted array
 *
 * @example
 * const numbers = [4, 2, 7, 1, 3];
 * insertionSort(numbers); // [1, 2, 3, 4, 7]
 *
 * @example
 * const numbers = [4, 2, 7, 1, 3];
 * insertionSort(numbers, (a, b) => a - b); // [1, 2, 3, 4, 7]
 *
 * @example
 * const numbers = [4, 2, 7, 1, 3];
 * insertionSort(numbers, undefined, 1, 3); // [4, 1, 2, 7, 3]
 */
export const insertionSort = <T>(
  array: T[],
  compareFunction: CompareFunction<T> = compareFunctionDefault<T>,
  start = 0,
  end: number = array.length - 1,
): T[] => {
  const result = [...array];
  insertionSortRange(result, compareFunction, start, end);
  return result;
};
