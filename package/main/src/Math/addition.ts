import { getDecimalLength } from "./getDecimalLength";

/**
 * Addition without floating point errors
 * @param  {number[]} numbers Numbers to add
 * @returns number Sum of the numbers
 * @example addition(0.1, 0.2); // 0.3
 */
export const addition = (...numbers: number[]) => {
  let maxDecimal = 0;
  let allIntegers = true;

  for (const number of numbers) {
    if (!Number.isInteger(number)) {
      allIntegers = false;
      const length = getDecimalLength(number);
      if (length > maxDecimal) {
        maxDecimal = length;
      }
    }
  }

  if (allIntegers) {
    return numbers.reduce((sum, current) => sum + current, 0);
  }

  const z = 10 ** maxDecimal;
  let sum = 0;
  for (const number of numbers) {
    sum += Math.round(number * z);
  }
  return sum / z;
};
