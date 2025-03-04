/**
 * Swaps two numbers to ensure x < y
 * @param  {number} x First number
 * @param  {number} y Second number
 * @return {[number, number]} Array with numbers in ascending order
 * @example valueSwap(2, 1); // [1, 2]
 * @example valueSwap(1, 2); // [1, 2]
 */
export const valueSwap = (x: number, y: number): [number, number] => {
  return x < y ? [x, y] : [y, x];
};
