import { isDouble } from "./isDouble";
import { isNumber } from "./isNumber";
/**
 * 最大値の位で区切ります。
 * @param  {string|number} number
 * @returns string
 * @example mathSeparator(1250); // [1000, 250]
 */
export const mathSeparator = (number: string | number): [number, number] => {
  let decimalPart = 0;
  if (isDouble(number)) {
    const splitNumber = String(number).split(".");
    // 小数点部分を取得
    decimalPart = Number(`0.${splitNumber[1]}`);
    number = splitNumber[0];
  }
  if (isNumber(number)) {
    const [n, x] =
      typeof number === "string"
        ? [number.length - 1, Number(number)]
        : [String(number).length - 1, number];
    if (n) {
      return [10 ** n, x - 10 ** n + decimalPart];
    }
    return [Number(number), 0];
  }
  return [0, 0];
};
