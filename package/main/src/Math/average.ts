import { addition } from "./addition";
import { division } from "./division";
/**
 * Calculates the arithmetic mean of an array of numbers
 * @param  {number[]} numbers - Array of numbers to average
 * @returns {number} The arithmetic mean, returns 0 for empty array
 * @example
 * average([1, 2, 3]); // 2
 * average([10, 20]); // 15
 * average([]); // 0
 */
export const average = (numbers: number[]): number => {
  if (numbers.length === 0) {
    return 0;
  }

  const sum = numbers.reduce((a, b) => addition(a, b), 0);
  const avg = division(sum, numbers.length);
  return avg;
};
