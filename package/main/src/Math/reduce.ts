import { gcd } from "./gcd";
/**
 * Reduces a fraction to its lowest terms
 * @param  {number} x Numerator
 * @param  {number} y Denominator
 * @returns {{x: number, y: number, gcd: number}} Reduced fraction and the GCD
 * @example reduce(2, 4); // {x: 1, y: 2, gcd: 2}
 * @description
 * Returns an object containing:
 * - x: reduced numerator
 * - y: reduced denominator
 * - gcd: greatest common divisor used for reduction
 */
export const reduce = (x: number, y: number) => {
  if (y === 0) {
    return { x: Number.NaN, y: Number.NaN };
  }
  if (x === 0) {
    return { x: 0, y: 1, gcd: Math.abs(y) };
  }
  const gcdValue = gcd(Math.abs(x), Math.abs(y));
  const sign = y < 0 ? -1 : 1;
  return {
    x: (x / gcdValue) * sign,
    y: Math.abs(y / gcdValue),
    gcd: gcdValue,
  };
};
