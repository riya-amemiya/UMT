/**
 * Computes quotient and remainder of division
 * @param  {number} x Dividend
 * @param  {number} y Divisor
 * @returns {number[]} Array containing [quotient, remainder]
 * @example quotient(5, 2); // [2, 1]
 * @description
 * Returns an array where the first element is the quotient (integer division result)
 * and the second element is the remainder
 */
export const quotient = (x: number, y: number) => [
  (x - (x % y)) / y,
  (x % y) + 0,
];
