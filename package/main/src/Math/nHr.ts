import { nCr } from "./nCr";

/**
 * Calculates combinations with repetition (nHr) - ways to choose r items from n items with repetition allowed
 * @param n - Total number of items
 * @param r - Number of items to choose
 * @returns {number} Number of combinations with repetition, or NaN for invalid arguments
 * @example nHr(5, 2); // 15
 * @description
 * Also known as "stars and bars" problem or "multichoose".
 * Uses the formula nHr = (n+r-1)Cr
 */
export const nHr = (n: number, r: number): number => {
  if (n === 0 || r === 0 || n < 0 || r < 0) {
    return Number.NaN;
  }

  const result = nCr(n + r - 1, r);

  return result;
};
