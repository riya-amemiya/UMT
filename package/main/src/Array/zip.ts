import type { ZipArrayType } from "$/array/zip";

/**
 * Creates a new array by combining elements from multiple arrays at corresponding positions
 * @param {T} arrays List of arrays to combine
 * @returns {ZipArrayType<T>} New array with combined elements from each input array
 * @example
 * zip([1, 2], ['a', 'b']); // [[1, 'a'], [2, 'b']]
 * zip([1, 2, 3], ['a', 'b']); // [[1, 'a'], [2, 'b']]
 */
export const zip = <T extends unknown[][]>(...arrays: T): ZipArrayType<T> => {
  const arraysLength = arrays.length;
  if (arraysLength === 0) {
    return [] as unknown as ZipArrayType<T>;
  }

  // Optimize: Avoid Math.min(...arrays.map(...)) to prevent Call Stack Size Exceeded errors
  // for a large number of arrays, and avoid intermediate array allocations.
  let minLength = arrays[0].length;
  for (let index = 1; index < arraysLength; index += 1) {
    if (arrays[index].length < minLength) {
      minLength = arrays[index].length;
    }
  }

  const result: unknown[][] = new Array(minLength);
  for (let index = 0; index < minLength; index += 1) {
    const tuple: unknown[] = new Array(arraysLength);
    for (let arrayIndex = 0; arrayIndex < arraysLength; arrayIndex += 1) {
      tuple[arrayIndex] = arrays[arrayIndex][index];
    }
    result[index] = tuple;
  }

  return result as unknown as ZipArrayType<T>;
};
