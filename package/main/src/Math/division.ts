import { getDecimalLength } from "./getDecimalLength";
import { valueSwap } from "./valueSwap";

/**
 * 誤差のない割り算
 * @param  {number} x
 * @param  {number} y
 * @param  {boolean} [isFloor=true]
 * @returns number
 * @example division(0.1, 0.2); // 0.5
 * @example division(10, 3, false); // [3, 1]
 */
export const division = <T extends boolean = true>(
  x: number,
  y: number,
  isFloor: T = true as T,
): T extends true ? number : number[] => {
  const [decimalLengthX, decimalLengthY] = valueSwap(
    getDecimalLength(x),
    getDecimalLength(y),
  );
  const n =
    decimalLengthX === decimalLengthY
      ? 1
      : 10 ** (decimalLengthY - decimalLengthX);
  x = +`${x}`.replace(".", "");
  y = +`${y}`.replace(".", "");
  return (
    isFloor
      ? x > y
        ? x / y / n
        : (x / y) * n
      : [x > y ? (x - (x % y)) / y / n : ((x - (x % y)) / y) * n, x % y]
  ) as T extends true ? number : number[];
};
