import { addition } from "./addition";
import { getDecimalLength } from "./getDecimalLength";
import { max } from "./max";
import { multiplication } from "./multiplication";

/**
 * Performs subtraction with arbitrary number of arguments without floating point errors
 * @param  {number[]} numbers Array of numbers to subtract
 * @returns {number} The result of the subtraction
 * @example subtract(0.1, 0.2); // -0.1
 * @example subtract(1, 0.1, 0.2); // 0.7
 * @description
 * This function handles floating point precision issues by scaling the numbers
 * to integers before performing subtraction, then scaling back to the original
 * decimal places. The first argument is the minuend, and all subsequent arguments
 * are subtracted from it.
 */

export const subtract = (...numbers: number[]): number => {
  if (numbers.length <= 1) {
    return numbers[0] ?? 0;
  }
  const [first, ...rest] = numbers;
  const restSum = addition(...rest);
  const z = 10 ** max(getDecimalLength(first), getDecimalLength(restSum));
  return (multiplication(first, z) - multiplication(restSum, z)) / z;
};
