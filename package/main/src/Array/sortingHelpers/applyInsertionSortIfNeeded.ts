import type { CompareFunction } from "$/array/compareFunction";
import { insertionSort } from "@/Array/insertionSort";

/**
 * Checks if a partition is small enough to apply insertion sort and applies it if so.
 *
 * @template T The type of elements in the array.
 * @param {T[]} array The array containing the partition.
 * @param {number} low The starting index of the partition.
 * @param {number} high The ending index of the partition.
 * @param {CompareFunction<T>} compareFunction The function to compare elements.
 * @param {number} insertionSortThreshold The size threshold for switching to insertion sort.
 * @returns {boolean} True if insertion sort was applied, false otherwise.
 */
export const applyInsertionSortIfNeeded = <T>(
  array: T[],
  low: number,
  high: number,
  compareFunction: CompareFunction<T>,
  insertionSortThreshold: number,
): boolean => {
  if (high - low + 1 <= insertionSortThreshold) {
    insertionSort(array, compareFunction, low, high);
    return true;
  }
  return false;
};
