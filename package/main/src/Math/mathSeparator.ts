import { isNumber } from "@/Validate/isNumber";

/**
 * 最大値の位で区切ります。
 * @param  {string|number} input
 * @returns [number, number]
 * @example mathSeparator(1250); // [1000, 250]
 */
export const mathSeparator = (input: string | number): [number, number] => {
  if (!isNumber(input)) {
    return [0, 0];
  }

  const [integerPart, fractionalPart] = String(input).split(".");
  const decimalPart = fractionalPart ? Number(`0.${fractionalPart}`) : 0;

  const numberOfDigits = integerPart.length - 1;
  const numericalValue = Number(integerPart);

  if (numberOfDigits === 0) {
    return [numericalValue, decimalPart];
  }

  const primary = 10 ** numberOfDigits;
  const remainder = numericalValue - primary + decimalPart;

  return [primary, remainder];
};
