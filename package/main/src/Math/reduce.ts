import { gcd } from "./gcd";
/**
 * 約分
 * @param  {number} x
 * @param  {number} y
 * @returns {x: number, y: number, gcd: number}
 * @example reduce(2, 4); // {x: 1, y: 2, gcd: 2}
 */
export const reduce = (x: number, y: number) => {
  if (x === 0 || y === 0) {
    return { x: Number.NaN, y: Number.NaN };
  }
  const n = gcd(x, y);
  return { x: x / n, y: y / n, gcd: gcd(x, y) };
};
