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
  if (arrays.length === 0) {
    return [] as unknown as ZipArrayType<T>;
  }
  const length = Math.min(...arrays.map((array) => array.length));
  return Array.from({ length }, (_, index) => {
    return arrays.map((array) => array[index]);
  }) as unknown as ZipArrayType<T>;
};
