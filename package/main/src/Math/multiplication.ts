import { addition } from "./addition";
import { getDecimalLength } from "./getDecimalLength";

const multiplyIntegers = (numbers: number[]): number => {
  let product = 1;
  for (const n of numbers) {
    product *= n;
  }
  return product;
};

/**
 * Performs multiplication without floating point errors for any number of arguments
 * @param  {...number[]} numbers Numbers to multiply
 * @returns {number} Product of all numbers
 * @example multiplication(0.1, 0.2, 0.3); // 0.006
 */
export const multiplication = (...numbers: number[]) => {
  if (numbers.every((n) => Number.isInteger(n))) {
    return multiplyIntegers(numbers);
  }

  const integers = numbers.map((n) => +`${n}`.replace(".", ""));
  const totalDecimal = addition(...numbers.map((n) => getDecimalLength(n)));
  return multiplyIntegers(integers) / 10 ** totalDecimal;
};
