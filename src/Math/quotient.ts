/**
 * あまりの割り算
 * @param  {number} x
 * @param  {number} y
 */
const quotient = (x: number, y: number) => [
    (x - (x % y)) / y,
    (x % y) + 0,
];
export default quotient;
