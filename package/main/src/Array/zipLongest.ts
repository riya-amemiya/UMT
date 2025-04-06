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
  const maxLength = Math.max(...arrays.map((array) => array.length));
  return Array.from({ length: maxLength }, (_, index) => {
    return arrays.map((array) => array[index]);
  }) as unknown as ZipArrayType<T>;
};
