import { getDecimalLength } from "./getDecimalLength";

/**
 * Performs division without floating point errors
 * @param  {number} x Dividend
 * @param  {number} y Divisor
 * @param  {boolean} [isFloor=true] If true, returns quotient; if false, returns [quotient, remainder]
 * @returns {number | number[]} Division result
 * @example division(0.1, 0.2); // 0.5
 * @example division(10, 3, false); // [3, 1]
 */
export const division = <T extends boolean = true>(
  x: number,
  y: number,
  isFloor: T = true as T,
): T extends true ? number : number[] => {
  if (y === 0) {
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    return isFloor ? (Number.NaN as any) : ([Number.NaN, Number.NaN] as any);
  }

  const sign = Math.sign(x) * Math.sign(y);
  const absX = Math.abs(x);
  const absY = Math.abs(y);

  // Get decimal lengths without swapping
  const decimalLengthX = getDecimalLength(absX);
  const decimalLengthY = getDecimalLength(absY);

  // Remove decimal points
  const xInt = +`${absX}`.replace(".", "");
  const yInt = +`${absY}`.replace(".", "");

  // Calculate scaling factor based on the actual decimal difference
  const scalingFactor = 10 ** (decimalLengthY - decimalLengthX);

  // Calculate the result
  const divisionResult = (xInt / yInt) * scalingFactor;

  if (isFloor) {
    // Apply sign to result
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    return (sign * divisionResult) as any;
  }
  // When isFloor is false, calculate quotient and remainder
  const intQuotient = Math.floor(divisionResult);
  const remainder = xInt % yInt;
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  return [sign * intQuotient, remainder] as any;
};
