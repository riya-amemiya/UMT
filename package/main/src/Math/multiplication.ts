import { getDecimalLength } from "./getDecimalLength";

/**
 * 誤差のない掛け算を任意長の引数で行う
 * @param  {...number[]} numbers
 * @returns number
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
