import { getDecimalLength } from "./getDecimalLength";

/**
 * Performs multiplication without floating point errors for any number of arguments
 * @param  {...number[]} numbers Numbers to multiply
 * @returns {number} Product of all numbers
 * @example multiplication(0.1, 0.2, 0.3); // 0.006
 */
export const multiplication = (...numbers: number[]) => {
  return numbers.reduce((accumulator, number) => {
    const n = 10 ** (getDecimalLength(accumulator) + getDecimalLength(number));
    const accumulatorWithoutDot = +`${accumulator}`.replace(".", "");
    const numberWithoutDot = +`${number}`.replace(".", "");
    return (accumulatorWithoutDot * numberWithoutDot) / n;
  }, 1);
};
