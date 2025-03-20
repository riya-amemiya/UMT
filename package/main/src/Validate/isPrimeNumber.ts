/**
 * Determines if a number is prime
 * @param  {number} n - Number to check (must be an integer)
 * @returns {boolean} true if the number is prime, false otherwise
 * @example isPrimeNumber(2); // true
 * isPrimeNumber(17); // true
 * isPrimeNumber(4); // false
 * isPrimeNumber(1); // false
 * isPrimeNumber(-3); // false
 */
export const isPrimeNumber = (n: number): boolean => {
  if (n <= 1 || !Number.isInteger(n)) {
    return false;
  }

  for (let index = 2; index <= Math.sqrt(n); index++) {
    if (n % index === 0) {
      return false;
    }
  }

  return true;
};
