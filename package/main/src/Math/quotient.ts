/**
 * あまりの割り算
 * @param  {number} x
 * @param  {number} y
 * @returns number[]
 * @example quotient(5, 2); // [2, 1]
 */
export const quotient = (x: number, y: number) => [
  (x - (x % y)) / y,
  (x % y) + 0,
];
