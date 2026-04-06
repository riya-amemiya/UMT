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

  // Handle small primes and eliminate even numbers / multiples of 3 early
  if (n <= 3) {
    return true;
  }
  if (n % 2 === 0 || n % 3 === 0) {
    return false;
  }

  // Performance: only test divisors of the form 6k +/- 1.
  // All primes > 3 are of this form, so we skip ~2/3 of candidates
  // compared to the naive loop that checks every integer from 2 to sqrt(n).
  for (let index = 5; index * index <= n; index += 6) {
    if (n % index === 0 || n % (index + 2) === 0) {
      return false;
    }
  }

  return true;
};
