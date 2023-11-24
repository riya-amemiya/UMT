import { getDecimalLength } from "./getDecimalLength";
import { valueSwap } from "./valueSwap";

export interface DIVISION {
  (x: number, y: number, isFloor?: true): number;
  (x: number, y: number, isFloor?: false): number[];
}
/**
 * 誤差のない割り算
 * @param  {number} x
 * @param  {number} y
 * @param  {boolean} [isFloor=true]
 * @returns number
 * @example division(0.1, 0.2); // 0.5
 */
export const division = ((x: number, y: number, isFloor = true) => {
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
  return isFloor
    ? x > y
      ? x / y / n
      : (x / y) * n
    : [x > y ? (x - (x % y)) / y / n : ((x - (x % y)) / y) * n, x % y];
}) as DIVISION;
