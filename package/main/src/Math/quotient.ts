/**
 * あまりの割り算
 * @param  {number} x
 * @param  {number} y
 */
export const quotient = (x: number, y: number) => [
    (x - (x % y)) / y,
    (x % y) + 0,
];
