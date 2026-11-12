/**
Calculates combinations (nCr) - number of ways to choose r items from n items
@param n - Total number of items
@param r - Number of items to choose
@returns {number} Number of combinations, Infinity when the result exceeds Number.MAX_VALUE, or NaN for invalid arguments
@example nCr(5, 2); // 10
@example nCr(1000, 500); // 2.7028824094543666e+299
@description
Calculates the number of ways to choose r items from a set of n items,
where the order doesn't matter. Uses the multiplicative formula
nCr = (n - k + 1) / 1 * (n - k + 2) / 2 * ... * n / k with k = min(r, n - r),
so the result stays finite whenever it fits in a double.
*/
export const nCr = (n: number, r: number): number => {
  if (n < r || n < 0 || r < 0) {
    return Number.NaN;
  }
  if (r === 0 || n === r) {
    return 1;
  }

  const effectiveR = Math.min(r, n - r);
  const base = n - effectiveR;

  let result = 1;
  for (let index = 1; index <= effectiveR; index++) {
    const factor = base + index;
    result =
      result > Number.MAX_VALUE / factor
        ? (result / index) * factor
        : (result * factor) / index;
    if (result === Number.POSITIVE_INFINITY) {
      return result;
    }
  }

  return result;
};
