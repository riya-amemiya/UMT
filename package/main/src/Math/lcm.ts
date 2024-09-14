import { division } from "./division";
import { gcd } from "./gcd";
import { multiplication } from "./multiplication";
import { valueSwap } from "./valueSwap";
/**
 * 最小公倍数
 * @param  {number} x
 * @param  {number} y
 * @returns number
 * @example lcm(2, 3); // 6
 */
export const lcm = (x: number, y: number) => {
  if (x === 0 || y === 0) {
    return 0;
  }
  const [newX, newY] = valueSwap(x, y);
  return multiplication(division(newX, gcd(newX, newY)), newY);
};
