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
  [x, y] = valueSwap(x, y);
  return multiplication(division(x, gcd(x, y)), y);
};
