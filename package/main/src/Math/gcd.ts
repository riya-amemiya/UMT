import { isDouble } from "@/Validate/isDouble";

/**
 * Greatest Common Divisor (GCD)
 * @param  {number} x First number
 * @param  {number} y Second number
 * @param  {number[]} z Additional numbers (optional)
 * @returns number The GCD of all input numbers
 * @example gcd(12, 18); // 6
 */
export const gcd = (x: number, y: number, ...z: number[]) => {
  // Handle decimal numbers by scaling them to integers
  const allNumbers = [x, y, ...z];

  // If any number is decimal, find common decimal places and scale
  const hasDecimals = allNumbers.some((number_) => isDouble(number_, false));

  if (hasDecimals) {
    // Find maximum decimal places
    const getDecimalPlaces = (number_: number) => {
      const string_ = number_.toString();
      return string_.includes(".") ? string_.split(".")[1].length : 0;
    };

    const maxDecimalPlaces = Math.max(
      ...allNumbers.map((number_) => getDecimalPlaces(number_)),
    );
    const multiplier = 10 ** maxDecimalPlaces;

    // Scale all numbers to integers
    const scaledNumbers = allNumbers.map((number_) =>
      Math.round(number_ * multiplier),
    );
    const result = gcdInteger(
      scaledNumbers[0],
      scaledNumbers[1],
      ...scaledNumbers.slice(2),
    );

    // Scale back to original decimal range
    return result / multiplier;
  }

  return gcdInteger(x, y, ...z);
};

const gcdInteger = (x: number, y: number, ...z: number[]) => {
  let copyX = Math.abs(Math.round(x));
  let copyY = Math.abs(Math.round(y));
  const copyZ = z.map((element) => Math.abs(Math.round(element)));

  if (copyX === 0) {
    return copyY;
  }
  if (copyY === 0) {
    return copyX;
  }

  [copyX, copyY] = [Math.max(copyX, copyY), Math.min(copyX, copyY)];

  /* Euclidean algorithm */
  let r = copyY % copyX;
  while (r !== 0) {
    copyY = copyX;
    copyX = r;
    r = copyY % copyX;
  }

  if (copyZ.length > 0) {
    for (const element of copyZ) {
      copyX = gcdInteger(copyX, element);
    }
  }

  return copyX;
};
