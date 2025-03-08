/**
 * Prime factorization of a number
 * @param  {number} n Number to factorize
 * @returns number[] Array of prime factors
 * @example factorize(12); // [2, 2, 3]
 */
export const factorize = (n: number): number[] => {
  const result: number[] = [];
  let remaining = Math.abs(n);

  for (let factor = 2; factor * factor <= remaining; factor++) {
    while (remaining % factor === 0) {
      result.push(factor);
      remaining /= factor;
    }
  }

  if (remaining > 1) {
    result.push(remaining);
  }

  return result;
};
