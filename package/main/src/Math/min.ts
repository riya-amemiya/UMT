/**
 * Returns the minimum value from the input numbers
 * @param {number[]} number_ Array of numbers
 * @returns {number} Minimum value
 * @example min(1, 2, 3); // 1
 * @description Automatically removes duplicates using Set
 */
export const min = (...number_: number[]) =>
  Reflect.apply(Math.min, null, [...new Set(number_)]);
