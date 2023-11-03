import { addition } from "./addition";
import { division } from "./division";
/**
 * 平均値
 * @param  {number[]} numbers
 * @returns number
 * @example average([1, 2, 3]); // 2
 */
export const average = (numbers: number[]): number => {
  if (numbers.length === 0) {
    return 0;
  }

  const sum = numbers.reduce((a, b) => addition(a, b), 0);
  const avg = division(sum, numbers.length);
  return avg;
};
