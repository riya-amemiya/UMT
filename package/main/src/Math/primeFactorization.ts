/**
 * Performs prime factorization of a number
 * @param  {number} x Number to factorize
 * @returns {Array<{number: number; count: number}>} Array of prime factors and their counts
 * @example primeFactorization(12); // [{number: 2, count: 2}, {number: 3, count: 1}]
 * @description
 * Returns an array of objects containing prime factors and their counts.
 * For example, 12 = 2^2 * 3^1 is represented as [{number: 2, count: 2}, {number: 3, count: 1}]
 */
export const primeFactorization = (x: number) => {
  let n = 0;
  let copyX = x;
  const out: { number: number; count: number }[] = [];
  for (let index = 2; index * index <= copyX; index++) {
    if (copyX % index === 0) {
      n = 0;
      while (copyX % index === 0) {
        n++;
        copyX /= index;
      }
      out.push({ number: index, count: n });
    }
  }
  // If remaining value is greater than 1, it's a prime factor
  if (copyX > 1) {
    out.push({ number: copyX, count: 1 });
  }
  return out;
};
