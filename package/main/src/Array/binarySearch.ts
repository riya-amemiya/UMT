/**
 * Binary search implementation
 * @param array A sorted array of numbers
 * @param target The value to search for
 * @returns The index of the target value in the array, or -1 if not found
 */
export const binarySearch = (array: number[], target: number): number => {
  let left = 0;
  let right = array.length - 1;

  while (left <= right) {
    const mid = Math.floor((left + right) / 2);
    const midValue = array[mid];

    if (midValue === target) {
      return mid;
    }
    if (midValue < target) {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }

  return -1;
};
