/**
 * Calculates permutations (nPr) - number of ways to arrange r items from n items
 * @param n - Total number of items
 * @param r - Number of items to arrange
 * @returns {number} Number of permutations, or NaN for invalid arguments
 * @example nPr(5, 2); // 20
 * @description
 * Calculates permutations where order matters, using the formula:
 * nPr = n * (n-1) * (n-2) * ... * (n-r+1)
 */
export const nPr = (n: number, r: number): number => {
  if (n < r || n < 0 || r < 0) {
    return Number.NaN;
  }
  if (r === 0) {
    return 1;
  }
  let result = 1;
  for (let index = 0; index < r; index++) {
    result *= n - index;
  }
  return result;
};
