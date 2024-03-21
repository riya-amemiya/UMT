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
  const sign = Math.sign(x) * Math.sign(y);
  const [decimalLengthX, decimalLengthY] = valueSwap(
    getDecimalLength(Math.abs(x)),
    getDecimalLength(Math.abs(y)),
  );
  const n =
    decimalLengthX === decimalLengthY
      ? 1
      : 10 ** (decimalLengthY - decimalLengthX);
  x = +`${Math.abs(x)}`.replace(".", "");
  y = +`${Math.abs(y)}`.replace(".", "");
  return (
    isFloor
      ? sign * (x > y ? x / y / n : (x / y) * n) // 結果に符号を適用
      : [
          sign * (x > y ? (x - (x % y)) / y / n : ((x - (x % y)) / y) * n),
          x % y,
        ]
  ) as T extends true ? number : number[]; // 結果に符号を適用
};
