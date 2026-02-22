import { getDecimalLength } from "./getDecimalLength";
import { max } from "./max";
import { multiplication } from "./multiplication";
/**
 * Addition without floating point errors
 * @param  {number[]} numbers Numbers to add
 * @returns number Sum of the numbers
 * @example addition(0.1, 0.2); // 0.3
 */
export const addition = (...numbers: number[]) => {
  const z = 10 ** max(...numbers.map((element) => getDecimalLength(element)));
  return (
    numbers.reduce((sum, current) => sum + multiplication(current, z), 0) / z
  );
};
