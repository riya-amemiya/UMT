import { division } from "./division";
import { gcd } from "./gcd";
import { multiplication } from "./multiplication";
import { valueSwap } from "./valueSwap";
/**
 * Least Common Multiple (LCM)
 * @param  {number} x First number
 * @param  {number} y Second number
 * @returns number The LCM of x and y
 * @example lcm(2, 3); // 6
 */
export const lcm = (x: number, y: number) => {
  if (x === 0 || y === 0) {
    return 0;
  }
  const [newX, newY] = valueSwap(Math.abs(x), Math.abs(y));
  return multiplication(division(newX, gcd(newX, newY)), newY);
};
