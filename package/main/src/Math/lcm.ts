import { division } from "./division";
import { gcd } from "./gcd";
import { multiplication } from "./multiplication";
import { valueSwap } from "./valueSwap";

const lcmPair = (x: number, y: number): number => {
  if (x === 0 || y === 0) {
    return 0;
  }
  const [newX, newY] = valueSwap(Math.abs(x), Math.abs(y));
  return multiplication(division(newX, gcd(newX, newY)), newY);
};

/**
Least Common Multiple (LCM)
@param  {number} x First number
@param  {number} y Second number
@param  {number[]} z Additional numbers (optional)
@returns number The LCM of all input numbers, or 0 if any of them is 0
@example lcm(2, 3); // 6
@example lcm(2, 3, 4); // 12
*/
export const lcm = (x: number, y: number, ...z: number[]): number => {
  let result = lcmPair(x, y);
  for (const value of z) {
    result = lcmPair(result, value);
  }
  return result;
};
