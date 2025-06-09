import { compareFunctionDefault } from "./compareFunctionDefault";

import type { CompareFunction } from "$/array/compareFunction";
import { insertionSort } from "@/Array/insertionSort";

const MIN_RUN = 32;

/**
 * Merges two sorted portions of the array
 * @param array Array containing the portions to merge
 * @param start Starting index of the first portion
 * @param mid Middle index separating the two portions
 * @param end Ending index of the second portion
 * @param compareFunction Function to compare elements
 */
const merge = <T>(
  array: T[],
  start: number,
  mid: number,
  end: number,
  compareFunction: CompareFunction<T>,
): void => {
  const left = array.slice(start, mid + 1);
  const right = array.slice(mid + 1, end + 1);
  let leftIndex = 0;
  let rightIndex = 0;
  let arrayIndex = start;

  while (leftIndex < left.length && rightIndex < right.length) {
    if (compareFunction(left[leftIndex], right[rightIndex]) <= 0) {
      array[arrayIndex] = left[leftIndex];
      leftIndex++;
    } else {
      array[arrayIndex] = right[rightIndex];
      rightIndex++;
    }
    arrayIndex++;
  }

  while (leftIndex < left.length) {
    array[arrayIndex] = left[leftIndex];
    leftIndex++;
    arrayIndex++;
  }

  while (rightIndex < right.length) {
    array[arrayIndex] = right[rightIndex];
    rightIndex++;
    arrayIndex++;
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
  const n = end - start + 1;
  const minRun = getMinRunLength(n);

  for (let index = start; index <= end; index += minRun) {
    insertionSort(
      array,
      compareFunction,
      index,
      Math.min(index + MIN_RUN - 1, end),
    );
  }

  for (let size = minRun; size < n; size *= 2) {
    for (let left = start; left <= end; left += 2 * size) {
      const mid = left + size - 1;
      const right = Math.min(left + 2 * size - 1, end);

      if (mid < right) {
        merge(array, left, mid, right, compareFunction);
      }
    }
  }

  return array;
};
