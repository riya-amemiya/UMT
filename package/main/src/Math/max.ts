/**
 * Returns the maximum value from the input numbers
 * @param {number[]} number_ Array of numbers
 * @returns number Maximum value
 * @example max(1, 2, 3); // 3
 * @description Automatically removes duplicates using Set
 */
export const max = (...number_: number[]) =>
  Reflect.apply(Math.max, null, [...new Set(number_)]);
