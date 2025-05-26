import { compareFunctionDefault } from "./compareFunctionDefault";
import { insertionSort } from "./insertionSort";
import { heapsort } from "./sortingHelpers/heapsort";
import { networkSort, medianOfThree } from "./sortingHelpers/networkSort";
import { partition3Way } from "./sortingHelpers/partition3Way";

import type { CompareFunction } from "$/array/compareFunction";

// Constants for algorithm selection
const INSERTION_SORT_THRESHOLD = 24;
const NINTHER_THRESHOLD = 128;

/**
 * Ultra-fast hybrid sorting algorithm optimized for maximum performance
 * @param array Array to sort
 * @param compareFunction Optional comparison function
 * @returns Sorted array
 */
export const ultraSort = <T>(
  array: T[],
  compareFunction: CompareFunction<T> = compareFunctionDefault<T>,
): T[] => {
  const length = array.length;

  if (length <= 1) {
    return array;
  }

  // For very small arrays, use network sort
  if (length <= 5) {
    networkSort(array, 0, length - 1, compareFunction);
    return array;
  }

  // For small arrays, use insertion sort
  if (length <= INSERTION_SORT_THRESHOLD) {
    insertionSort(array, compareFunction);
    return array;
  }

  // For larger arrays, use introsort with optimizations
  introsort(
    array,
    0,
    length - 1,
    compareFunction,
    2 * Math.floor(Math.log2(length)),
  );
  return array;
};

/**
 * Introsort implementation with depth limit
 */
const introsort = <T>(
  array: T[],
  low: number,
  high: number,
  compareFunction: CompareFunction<T>,
  depthLimit: number,
): void => {
  let currentLow = low;
  let currentHigh = high;
  let currentDepthLimit = depthLimit;

  while (currentHigh > currentLow) {
    if (currentHigh - currentLow + 1 <= INSERTION_SORT_THRESHOLD) {
      insertionSort(array, compareFunction, currentLow, currentHigh);
      return;
    }

    if (currentDepthLimit === 0) {
      heapsort(array, currentLow, currentHigh, compareFunction);
      return;
    }

    currentDepthLimit--;

    // Choose pivot using adaptive method
    const pivotIndex = choosePivot(
      array,
      currentLow,
      currentHigh,
      compareFunction,
    );

    // Three-way partitioning for better handling of duplicates
    const [lt, gt] = partition3Way(
      array,
      currentLow,
      currentHigh,
      pivotIndex,
      compareFunction,
    );

    // Recurse on smaller partition first (tail recursion optimization)
    if (lt - currentLow < currentHigh - gt) {
      introsort(array, currentLow, lt - 1, compareFunction, currentDepthLimit);
      currentLow = gt + 1;
    } else {
      introsort(array, gt + 1, currentHigh, compareFunction, currentDepthLimit);
      currentHigh = lt - 1;
    }
  }
};

/**
 * Choose pivot using adaptive strategy
 */
const choosePivot = <T>(
  array: T[],
  low: number,
  high: number,
  compareFunction: CompareFunction<T>,
): number => {
  const length = high - low + 1;

  if (length < NINTHER_THRESHOLD) {
    // Use median-of-three for smaller arrays
    const mid = low + Math.floor(length / 2);
    return medianOfThree(array, low, mid, high, compareFunction);
  }

  // Use ninther (median of three medians) for larger arrays
  const gap = Math.floor(length / 8);
  const mid = low + Math.floor(length / 2);

  const m1 = medianOfThree(
    array,
    low,
    Math.min(low + gap, high),
    Math.min(low + 2 * gap, high),
    compareFunction,
  );
  const m2 = medianOfThree(
    array,
    Math.max(low, mid - gap),
    mid,
    Math.min(mid + gap, high),
    compareFunction,
  );
  const m3 = medianOfThree(
    array,
    Math.max(low, high - 2 * gap),
    Math.max(low, high - gap),
    high,
    compareFunction,
  );

  return medianOfThree(array, m1, m2, m3, compareFunction);
};
