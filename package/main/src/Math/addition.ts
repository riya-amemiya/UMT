import { getDecimalLength } from "./getDecimalLength";
import { max } from "./max";
import { multiplication } from "./multiplication";
/**
 * 誤差のない足し算
 * @param  {number} x
 * @param  {number} y
 * @returns number
 * @example addition(0.1, 0.2); // 0.3
 */
export const addition = (x: number, y: number) => {
  const z = 10 ** max(getDecimalLength(x), getDecimalLength(y));
  return (multiplication(x, z) + multiplication(y, z)) / z;
};
