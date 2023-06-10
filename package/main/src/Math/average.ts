import { addition } from "./addition";
import { division } from "./division";
/**
 * 平均値
 * @param  {number[]} numbers
 * @returns number
 */
export const average = (numbers: number[]): number => {
  const sum = numbers.reduce((a, b) => addition(a, b), 0);
  const avg = division(sum, numbers.length);
  return avg;
};
