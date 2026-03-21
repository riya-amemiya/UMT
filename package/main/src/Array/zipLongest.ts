import type { ZipArrayType } from "$/array/zip";

/**
 * Combines arrays of different lengths by padding shorter arrays with undefined values
 * to match the length of the longest array
 *
 * @param {T} arrays List of arrays to combine
 * @returns {ZipArrayType<T>} New array with combined elements from each input array,
 *                            padded with undefined values where necessary
 * @example
 * zipLongest([1, 2], ['a']); // [[1, 'a'], [2, undefined]]
 * zipLongest([1], ['a', 'b']); // [[1, 'a'], [undefined, 'b']]
 */
export const zipLongest = <T extends unknown[][]>(
  ...arrays: T
): ZipArrayType<T> => {
  const arraysLength = arrays.length;
  if (arraysLength === 0) {
    return [] as unknown as ZipArrayType<T>;
  }

  // Optimize: Avoid Math.max(...arrays.map(...)) to prevent Call Stack Size Exceeded errors
  // for a large number of arrays, and avoid intermediate array allocations.
  let maxLength = arrays[0].length;
  for (let index = 1; index < arraysLength; index += 1) {
    if (arrays[index].length > maxLength) {
      maxLength = arrays[index].length;
    }
  }

  // Optimize: Pre-allocate arrays with known lengths and use direct index assignment
  // instead of .push() to avoid repeated capacity checks and potential reallocations.
  const result: unknown[][] = new Array(maxLength);
  for (let index = 0; index < maxLength; index += 1) {
    const tuple: unknown[] = new Array(arraysLength);
    for (let arrayIndex = 0; arrayIndex < arraysLength; arrayIndex += 1) {
      tuple[arrayIndex] = arrays[arrayIndex][index];
    }
    result[index] = tuple;
  }

  return result as unknown as ZipArrayType<T>;
};
