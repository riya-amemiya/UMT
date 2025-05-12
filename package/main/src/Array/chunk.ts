import type { ChunkArrayType } from "$/array/chunk"; /**
 * Split an array into smaller chunks of specified size
 * @param {T} array The array to split
 * @param {N} n The size of each chunk
 * @returns Array of chunks
 * @example chunk([1, 2, 3, 4, 5, 6, 7, 8, 9], 3); // [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
 */
export const chunk = <T extends unknown[], N extends number>(
  array: T,
  n: N,
): ChunkArrayType<T, N> => {
  const length = array.length;
  // eslint-disable-next-line unicorn/no-new-array
  const result = new Array(Math.ceil(length / n));

  for (let index = 0, k = 0; index < length; index += n, k++) {
    result[k] = array.slice(index, index + n) as unknown as T[];
  }

  return result as ChunkArrayType<T, N>;
};
