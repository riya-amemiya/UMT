import type { CompareFunction } from "$/array/compareFunction";

/**
 * Heapsort implementation for worst-case guarantee
 */
export const heapsort = <T>(
  array: T[],
  low: number,
  high: number,
  compareFunction: CompareFunction<T>,
): void => {
  const n = high - low + 1;

  // Build max heap
  for (let index = Math.floor(n / 2) - 1; index >= 0; index--) {
    heapify(array, n, index, low, compareFunction);
  }

  // Extract elements from heap
  for (let index = n - 1; index > 0; index--) {
    [array[low], array[low + index]] = [array[low + index], array[low]];
    heapify(array, index, 0, low, compareFunction);
  }
};

/**
 * Heapify subtree
 */
const heapify = <T>(
  array: T[],
  n: number,
  index: number,
  offset: number,
  compareFunction: CompareFunction<T>,
): void => {
  let largest = index;
  const left = 2 * index + 1;
  const right = 2 * index + 2;

  if (
    left < n &&
    compareFunction(array[offset + left], array[offset + largest]) > 0
  ) {
    largest = left;
  }

  if (
    right < n &&
    compareFunction(array[offset + right], array[offset + largest]) > 0
  ) {
    largest = right;
  }

  if (largest !== index) {
    [array[offset + index], array[offset + largest]] = [
      array[offset + largest],
      array[offset + index],
    ];
    heapify(array, n, largest, offset, compareFunction);
  }
};
