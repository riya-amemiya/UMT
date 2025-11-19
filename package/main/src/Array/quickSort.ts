import { compareFunctionDefault } from "./compareFunctionDefault";
import { applyInsertionSortIfNeeded } from "./sortingHelpers/applyInsertionSortIfNeeded";
import { validateRange } from "./sortingHelpers/rangeValidator";

import type { CompareFunction } from "$/array/compareFunction";

/**
 * Finds the median value among three elements in the array
 * @param array The array containing the elements
 * @param a Index of first element
 * @param b Index of second element
 * @param c Index of third element
 * @param compareFunction Function to compare elements
 * @returns The median value among the three elements
 */
const medianOfThree = <T>(
  array: T[],
  a: number,
  b: number,
  c: number,
  compareFunction: CompareFunction<T>,
): T => {
  const ab = compareFunction(array[a], array[b]);
  if (ab < 0) {
    const bc = compareFunction(array[b], array[c]);
    if (bc < 0) {
      return array[b];
    }
    if (compareFunction(array[a], array[c]) < 0) {
      return array[c];
    }
    return array[a];
  }
  const ac = compareFunction(array[a], array[c]);
  if (ac < 0) {
    return array[a];
  }
  if (compareFunction(array[b], array[c]) < 0) {
    return array[c];
  }
  return array[b];
};

/**
 * Partitions the array around a pivot element using median-of-three strategy
 * @param array Array to partition
 * @param low Starting index of the partition range
 * @param high Ending index of the partition range
 * @param compareFunction Function to compare elements
 * @returns Index of the partition point
 */
const partition = <T>(
  array: T[],
  low: number,
  high: number,
  compareFunction: CompareFunction<T>,
): number => {
  const pivot = medianOfThree(
    array,
    low,
    Math.floor((low + high) / 2),
    high,
    compareFunction,
  );
  let left = low - 1;
  let right = high + 1;

  while (true) {
    do {
      left++;
    } while (compareFunction(array[left], pivot) < 0);
    do {
      right--;
    } while (compareFunction(array[right], pivot) > 0);

    if (left >= right) {
      return right;
    }

    [array[left], array[right]] = [array[right], array[left]];
  }
};

/**
 * Internal implementation of the quicksort algorithm with tail-call optimization
 * @param array Array to sort
 * @param lowInit Initial low index of the range to sort
 * @param highInit Initial high index of the range to sort
 * @param compareFunction Function to compare elements
 * @param insertionSortThreshold Size threshold for switching to insertion sort
 */
const sortImpl = <T>(
  array: T[],
  lowInit: number,
  highInit: number,
  compareFunction: CompareFunction<T>,
  insertionSortThreshold: number,
): void => {
  let low = lowInit;
  let high = highInit;

  while (low < high) {
    if (
      applyInsertionSortIfNeeded(
        array,
        low,
        high,
        compareFunction,
        insertionSortThreshold,
      )
    ) {
      return;
    }
    const pivotIndex = partition(array, low, high, compareFunction);
    if (pivotIndex - low < high - pivotIndex) {
      sortImpl(array, low, pivotIndex, compareFunction, insertionSortThreshold);
      low = pivotIndex + 1;
    } else {
      sortImpl(
        array,
        pivotIndex + 1,
        high,
        compareFunction,
        insertionSortThreshold,
      );
      high = pivotIndex;
    }
  }
};

/**
 * Sorts an array using a hybrid algorithm combining QuickSort and InsertionSort
 * @param {T[]} array Array to sort
 * @param {CompareFunction<T>} compareFunction Comparison function that returns negative if a < b, zero if a = b, positive if a > b
 * @param {number} startIndex Starting index for the sort range (default: 0)
 * @param {number} endIndex Ending index for the sort range (default: array.length - 1)
 * @param {number} insertionSortThreshold Threshold for switching to insertion sort (default: 10)
 * @returns {T[]} Sorted array
 * @example
 * quickSort([1, 3, 2, 4, 5]); // [1, 2, 3, 4, 5]
 * quickSort([1, 3, 2], (a, b) => b - a); // [3, 2, 1]
 * quickSort(['b', 'a', 'c']); // ['a', 'b', 'c']
 */
export const quickSort = <T>(
  array: T[],
  compareFunction: CompareFunction<T> = compareFunctionDefault<T>,
  startIndex = 0,
  endIndex = array.length - 1,
  insertionSortThreshold = 10,
): T[] => {
  const result = [...array];
  const {
    startIndex: validStartIndex,
    endIndex: validEndIndex,
    shouldSort,
  } = validateRange(result, startIndex, endIndex);

  if (shouldSort) {
    sortImpl(
      result,
      validStartIndex,
      validEndIndex,
      compareFunction,
      insertionSortThreshold,
    );
  }
  return result;
};
