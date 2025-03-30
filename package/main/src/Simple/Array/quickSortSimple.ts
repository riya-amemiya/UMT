import type { CompareFunction } from "$/array/compareFunction";
import { compareFunctionDefault } from "@/Array/compareFunctionDefault";
import { quickSort } from "@/Array/quickSort";

/**
 * Quick sort implementation for arrays
 * @template T Type of array elements
 * @param {T[]} array Input array to sort
 * @param {(a: T, b: T) => number} compareFunction Function to determine sort order
 * @param {number} startID Starting index for sorting
 * @param {number} endID Ending index for sorting
 * @returns {T[]} Sorted array
 * @example quickSort([1, 3, 2, 4, 5], (a, b) => a - b); // [1, 2, 3, 4, 5]
 */
export const quickSortSimple = <T>(
  array: T[],
  compareFunction: CompareFunction<T> = compareFunctionDefault<T>,
  startID = 0,
  endID: number = array.length - 1,
): T[] => {
  let localStartID = startID;
  let localEndID = endID;
  if (
    localStartID < 0 ||
    localStartID >= array.length ||
    localStartID > localEndID
  ) {
    localStartID = 0;
  }
  if (localEndID < 0 || localEndID >= array.length) {
    localEndID = array.length - 1;
  }
  return quickSort(array, compareFunction, localStartID, localEndID);
};
