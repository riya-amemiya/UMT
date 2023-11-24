import { gcd } from "./gcd";
import { valueSwap } from "./valueSwap";
/**
 * 最小公倍数
 * @param  {number} x
 * @param  {number} y
 * @returns number
 * @example lcm(2, 3); // 6
 */
export const lcm = (x: number, y: number) => {
  // If either input is 0, the least common multiple is 0
  if (x === 0 || y === 0) {
    return 0;
  }
  // Swap the values of x and y if x is greater than y
  [x, y] = valueSwap(x, y);
  // The least common multiple is x times y divided by their greatest common divisor
  return (x / gcd(x, y)) * y;
};
