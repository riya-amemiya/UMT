import { isDouble } from "./isDouble";
import { isNumber } from "./isNumber";
/**
 * 最大値の位で区切ります。
 * @param  {string|number} number
 * @returns string
 * @example mathSeparator(1250); // [1000, 250]
 */
export const mathSeparator = (number: string | number): [number, number] => {
  let copyNumber = number;
  let decimalPart = 0;
  if (isDouble(copyNumber)) {
    const splitNumber = String(copyNumber).split(".");
    // 小数点部分を取得
    decimalPart = Number(`0.${splitNumber[1]}`);
    copyNumber = splitNumber[0];
  }
  if (isNumber(copyNumber)) {
    const [n, x] =
      typeof copyNumber === "string"
        ? [copyNumber.length - 1, Number(copyNumber)]
        : [String(copyNumber).length - 1, copyNumber];
    if (n) {
      return [10 ** n, x - 10 ** n + decimalPart];
    }
    return [Number(copyNumber), 0];
  }
  return [0, 0];
};
