import { compareFunctionDefault } from "./compareFunctionDefault";
import { applyInsertionSortIfNeeded } from "./sortingHelpers/applyInsertionSortIfNeeded";
import { validateRange } from "./sortingHelpers/rangeValidator";

import type { CompareFunction } from "$/array/compareFunction";

interface PartitionResult {
  leftPivotIndex: number;
  rightPivotIndex: number;
}

/**
 * Get the median of three elements in the array
 * @param array - The input array
 * @param a - Index of the first element
 * @param b - Index of the second element
 * @param c - Index of the third element
 * @param compareFunction - Comparison function
 * @returns Index of the median element
 */
const medianOfThree = <T>(
  array: T[],
  a: number,
  b: number,
  c: number,
  compareFunction: CompareFunction<T>,
): number => {
  // Return the index of the median value
  const values = [
    { index: a, value: array[a] },
    { index: b, value: array[b] },
    { index: c, value: array[c] },
  ];

  // Sort and return the middle element's index
  values.sort((x, y) => compareFunction(x.value, y.value));
  return values[1].index;
};

/**
 * Select dual pivots and partition the array into three parts
 * @param array - Array to be partitioned
 * @param low - Starting index
 * @param high - Ending index
 * @param compareFunction - Comparison function
 * @returns Object containing left and right pivot indices
 */
const partition = <T>(
  array: T[],
  low: number,
  high: number,
  compareFunction: CompareFunction<T>,
): PartitionResult => {
  // Select pivot candidates
  const length = high - low;
  const gap = Math.max(1, Math.floor(length / 3));

  // Find optimal pivot positions
  const leftPivotIndex = medianOfThree(
    array,
    low,
    low + gap,
    Math.min(low + 2 * gap, high),
    compareFunction,
  );

  const rightPivotIndex = medianOfThree(
    array,
    Math.max(low, high - 2 * gap),
    high - gap,
    high,
    compareFunction,
  );

  // Move pivots to ends
  if (leftPivotIndex !== low) {
    [array[low], array[leftPivotIndex]] = [array[leftPivotIndex], array[low]];
  }
  if (rightPivotIndex !== high) {
    [array[high], array[rightPivotIndex]] = [
      array[rightPivotIndex],
      array[high],
    ];
  }

  // Swap pivots if needed
  if (compareFunction(array[low], array[high]) > 0) {
    [array[low], array[high]] = [array[high], array[low]];
  }

  // Initialize partition boundaries
  let left = low + 1;
  let right = high - 1;
  let current = left;

  // Partition into three parts
  while (current <= right) {
    // Handle elements smaller than left pivot
    if (compareFunction(array[current], array[low]) < 0) {
      [array[current], array[left]] = [array[left], array[current]];
      left++;
    }
    // Handle elements larger than right pivot
    else if (compareFunction(array[current], array[high]) > 0) {
      // Find suitable position from right
      while (
        current < right &&
        compareFunction(array[right], array[high]) > 0
      ) {
        right--;
      }
      [array[current], array[right]] = [array[right], array[current]];
      right--;
      // Check if swapped element is smaller than left pivot
      if (compareFunction(array[current], array[low]) < 0) {
        [array[current], array[left]] = [array[left], array[current]];
        left++;
      }
    }
    current++;
  }

  // Move pivots to their final positions
  left--;
  right++;
  [array[low], array[left]] = [array[left], array[low]];
  [array[high], array[right]] = [array[right], array[high]];

  return { leftPivotIndex: left, rightPivotIndex: right };
};

/**
 * Internal implementation of dual-pivot quicksort
 * @param array - Array to be sorted
 * @param start - Starting index
 * @param end - Ending index
 * @param compareFunction - Comparison function
 * @param insertionSortThreshold - Threshold for switching to insertion sort
 */
const sortRange = <T>(
  array: T[],
  start: number,
  end: number,
  compareFunction: CompareFunction<T>,
  insertionSortThreshold: number,
): void => {
  if (
    applyInsertionSortIfNeeded(
      array,
      start,
      end,
      compareFunction,
      insertionSortThreshold,
    )
  ) {
    return;
  }

  // Get partition indices
  const { leftPivotIndex, rightPivotIndex } = partition(
    array,
    start,
    end,
    compareFunction,
  );

  // Sort left partition
  sortRange(
    array,
    start,
    leftPivotIndex - 1,
    compareFunction,
    insertionSortThreshold,
  );

  // Sort middle partition
  if (rightPivotIndex - leftPivotIndex > 1) {
    sortRange(
      array,
      leftPivotIndex + 1,
      rightPivotIndex - 1,
      compareFunction,
      insertionSortThreshold,
    );
  }

  // Sort right partition
  sortRange(
    array,
    rightPivotIndex + 1,
    end,
    compareFunction,
    insertionSortThreshold,
  );
};

/**
 * Sort array using dual-pivot quicksort algorithm
 * More efficient than traditional quicksort for arrays with many duplicate values
 * @param array - Array to be sorted
 * @param compareFunction - Optional comparison function
 * @param startIndex - Optional starting index (default: 0)
 * @param endIndex - Optional ending index (default: array.length - 1)
 * @param insertionSortThreshold - Optional threshold for insertion sort (default: 10)
 * @returns Sorted array
 * @example
 * dualPivotQuickSort([3, 1, 4, 1, 5, 9, 2, 6, 5, 3]); // [1, 1, 2, 3, 3, 4, 5, 5, 6, 9]
 * dualPivotQuickSort(['banana', 'apple', 'orange']); // ['apple', 'banana', 'orange']
 */
export const dualPivotQuickSort = <T>(
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
    sortRange(
      result,
      validStartIndex,
      validEndIndex,
      compareFunction,
      insertionSortThreshold,
    );
  }
  return result;
};
