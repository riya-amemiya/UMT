import { isDouble } from "@/Validate/isDouble";
import { isNumber } from "@/Validate/isNumber";
/**
 * 最大値の位で区切ります。
 * @param  {string|number} number
 * @returns string
 * @example mathSeparator(1250); // [1000, 250]
 */
export const mathSeparator = (number: string | number): [number, number] => {
  let decimalPart = 0;

  if (!isNumber(number)) {
    return [0, 0];
  }

  if (isDouble(number)) {
    const [integerPart, fractionalPart] = String(number).split(".");
    decimalPart = Number(`0.${fractionalPart}`);
    number = integerPart;
  }

  const numberString = String(number);
  const numberOfDigits = numberString.length - 1;
  const numericalValue = Number(numberString);

  if (numberOfDigits === 0) {
    return [numericalValue, 0];
  }

  const primary = 10 ** numberOfDigits;
  const remainder = numericalValue - primary + decimalPart;

  return [primary, remainder];
};
