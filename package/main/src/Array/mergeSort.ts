import { compareFunctionDefault } from "./compareFunctionDefault";

import type { CompareFunction } from "$/array/compareFunction";

/**
 * Merge sort implementation
 * @param  {T[]} array Array to sort
 * @param  {(a: T, b: T) => number} compareFunction Comparison function
 * @returns {T[]} Sorted array
 * @example mergeSort([1, 3, 2, 4, 5], (a, b) => a - b); // [1, 2, 3, 4, 5]
 */
export const mergeSort = <T>(
  array: T[],
  compareFunction: CompareFunction<T> = compareFunctionDefault,
): T[] => {
  if (array.length <= 1) {
    return array;
  }

  const length = array.length;
  const result = array.slice();
  const aux = array.slice();

  mergeSortRecursive(result, aux, 0, length, compareFunction);

  return result;
};

/**
 * Recursive merge sort implementation
 * @param array Array to sort
 * @param aux Auxiliary array
 * @param start Start index
 * @param end End index
 * @param compareFunction Comparison function
 */
function mergeSortRecursive<T>(
  array: T[],
  aux: T[],
  start: number,
  end: number,
  compareFunction: (a: T, b: T) => number,
): void {
  if (end - start <= 1) {
    return;
  }

  const mid = (start + end) >>> 1;

  mergeSortRecursive(array, aux, start, mid, compareFunction);
  mergeSortRecursive(array, aux, mid, end, compareFunction);

  merge(array, aux, start, mid, end, compareFunction);
}

/**
 * Merges two sorted subarrays
 * @param array Array containing the subarrays
 * @param aux Auxiliary array
 * @param start Start index
 * @param mid Middle index
 * @param end End index
 * @param compareFunction Comparison function
 */
function merge<T>(
  array: T[],
  aux: T[],
  start: number,
  mid: number,
  end: number,
  compareFunction: (a: T, b: T) => number,
): void {
  for (let index = start; index < end; index++) {
    aux[index] = array[index];
  }

  let leftIndex = start;
  let rightIndex = mid;

  for (let k = start; k < end; k++) {
    if (leftIndex >= mid) {
      array[k] = aux[rightIndex++];
    } else if (rightIndex >= end) {
      array[k] = aux[leftIndex++];
    } else if (compareFunction(aux[leftIndex], aux[rightIndex]) <= 0) {
      array[k] = aux[leftIndex++];
    } else {
      array[k] = aux[rightIndex++];
    }
  }
}
