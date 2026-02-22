import { compareFunctionDefault } from "./compareFunctionDefault";
import { insertionSortRange } from "./sortingHelpers/insertionSortRange";

import type { CompareFunction } from "$/array/compareFunction";

const MIN_RUN = 32;

/**
 * Merges two sorted portions of the array
 * @param array Array containing the portions to merge
 * @param start Starting index of the first portion
 * @param mid Middle index separating the two portions
 * @param end Ending index of the second portion
 * @param compareFunction Function to compare elements
 * @param temp Temporary array for merging
 */
const merge = <T>(
  array: T[],
  start: number,
  mid: number,
  end: number,
  compareFunction: CompareFunction<T>,
  temporary: T[],
): void => {
  const length1 = mid - start + 1;

  // Optimization: check if already sorted
  if (compareFunction(array[mid], array[mid + 1]) <= 0) {
    return;
  }

  // Copy left run to temp
  for (let index = 0; index < length1; index++) {
    temporary[index] = array[start + index];
  }

  let index = 0; // index in temp (left run)
  let index_ = mid + 1; // index in array (right run)
  let k = start; // index in array (merge destination)

  while (index < length1 && index_ <= end) {
    if (compareFunction(temporary[index], array[index_]) <= 0) {
      array[k] = temporary[index];
      index++;
    } else {
      array[k] = array[index_];
      index_++;
    }
    k++;
  }

  while (index < length1) {
    array[k] = temporary[index];
    k++;
    index++;
  }
};

/**
 * Calculates the minimum length of a run for the given input size
 * This is used to determine the size of runs for the initial insertion sort phase
 * @param input Size of the array to be sorted
 * @returns Minimum length of a run
 */
const getMinRunLength = (input: number): number => {
  let n = input;
  let r = 0;
  while (n >= MIN_RUN) {
    r |= n & 1;
    n >>= 1;
  }
  return n + r;
};

/**
 * Implementation of the TimSort algorithm, which combines the best features of
 * insertion sort and merge sort. It provides a stable sort with O(n log n)
 * worst-case time complexity.
 *
 * @param {T[]} array Array to sort
 * @param {CompareFunction<T>} [compareFunction=compareFunctionDefault<T>]
 *        Function to compare elements
 * @param {number} [start=0] Starting index for the sort range
 * @param {number} [end=array.length - 1] Ending index for the sort range
 * @returns {T[]} Sorted array
 * @example
 * timSort([3, 1, 4, 1, 5]); // [1, 1, 3, 4, 5]
 * timSort(['b', 'a', 'c']); // ['a', 'b', 'c']
 */
export const timSort = <T>(
  array: T[],
  compareFunction: CompareFunction<T> = compareFunctionDefault<T>,
  start = 0,
  end: number = array.length - 1,
): T[] => {
  const result = [...array];
  const n = end - start + 1;
  const minRun = getMinRunLength(n);

  for (let runStart = start; runStart <= end; runStart += minRun) {
    const runEnd = Math.min(runStart + minRun - 1, end);
    insertionSortRange(result, compareFunction, runStart, runEnd);
  }

  // eslint-disable-next-line unicorn/no-new-array
  const temporary = new Array(n);

  for (let size = minRun; size < n; size *= 2) {
    for (let left = start; left <= end; left += 2 * size) {
      const mid = left + size - 1;
      const right = Math.min(left + 2 * size - 1, end);

      if (mid < right) {
        merge(result, left, mid, right, compareFunction, temporary);
      }
    }
  }

  return result;
};
