import type { First } from "$/logic/first";

/**
 * Returns the first element of an array
 * @param array The input array
 * @returns The first element of the array, or undefined if the array is empty
 * @example first([1, 2, 3]); // 1
 */
export const first = <T extends unknown[]>(
  array: T,
): First<T> extends never ? T[number] | undefined : First<T> => {
  return (array.length > 0 ? array[0] : undefined) as First<T> extends never
    ? T[number] | undefined
    : First<T>;
};
