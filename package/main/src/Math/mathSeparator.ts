import { isNumber } from "@/Validate/isNumber";

/**
 * Separates a number at its highest place value
 * @param  {string|number} input Value to separate
 * @returns [number, number] [primary value (highest place), remainder]
 * @example mathSeparator(1250); // [1000, 250]
 * @description
 * This function takes a number and separates it into two parts:
 * 1. The highest place value (e.g., 1000 for 1250)
 * 2. The remainder (e.g., 250 for 1250)
 * It also handles decimal numbers appropriately
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
