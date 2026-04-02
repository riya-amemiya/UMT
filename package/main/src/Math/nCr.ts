import { factorial } from "./factorial";
import { nPr } from "./nPr";

/**
 * Calculates combinations (nCr) - number of ways to choose r items from n items
 * @param n - Total number of items
 * @param r - Number of items to choose
 * @returns {number} Number of combinations, or NaN for invalid arguments
 * @example nCr(5, 2); // 10
 * @description
 * Calculates the number of ways to choose r items from a set of n items,
 * where the order doesn't matter. Uses the formula nCr = nPr / r!
 */
export const nCr = (n: number, r: number): number => {
  if (n < r || n < 0 || r < 0) {
    return Number.NaN;
  }
  if (r === 0 || n === r) {
    return 1;
  }

  // Performance: use symmetry property nCr(n, r) = nCr(n, n-r) to
  // minimize the number of multiplications when r > n/2.
  // e.g. nCr(100, 97) becomes nCr(100, 3), reducing 193 multiplications to 5.
  const effectiveR = Math.min(r, n - r);

  const numerator = nPr(n, effectiveR);
  const denominator = factorial(effectiveR);

  const result = numerator / denominator;

  return result;
};
