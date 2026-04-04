import { getDecimalLength } from "@/Math/getDecimalLength";

/**
 * Returns the sum of an array of numbers
 * @param {number[]} x Array of numbers
 * @returns Sum of the array elements
 * @example sum([1, 2, 3]); // 6
 */
export const sum = (x: number[]): number => {
  const length = x.length;

  if (length === 0) {
    return 0;
  }

  // Performance: determine decimal precision once for the entire array,
  // rather than calling addition() per element which redundantly checks
  // Number.isInteger() and getDecimalLength() on every accumulator step.
  // This reduces overhead from O(n) getDecimalLength calls on the
  // accumulator to a single pass over the input values.
  let maxDecimal = 0;

  for (let index = 0; index < length; index++) {
    if (!Number.isInteger(x[index])) {
      const decLength = getDecimalLength(x[index]);
      if (decLength > maxDecimal) {
        maxDecimal = decLength;
      }
    }
  }

  // Fast path: all integers, just sum directly
  if (maxDecimal === 0) {
    let result = 0;
    for (let index = 0; index < length; index++) {
      result += x[index];
    }
    return result;
  }

  // Decimal path: scale to integers, sum, then scale back.
  // Single multiplication factor computed once instead of per-element.
  const scale = 10 ** maxDecimal;
  let scaled = 0;
  for (let index = 0; index < length; index++) {
    scaled += Math.round(x[index] * scale);
  }
  return scaled / scale;
};
