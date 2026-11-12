/**
Determines whether a number is prime using 6k +/- 1 trial division
@param {number} n Number to check
@returns {boolean} true if n is a prime integer; false for non-integers, NaN, Infinity, and numbers below 2
@example isPrime(2); // true
@example isPrime(97); // true
@example isPrime(91); // false
@example isPrime(7.5); // false
*/
export const isPrime = (n: number): boolean => {
  if (!Number.isInteger(n) || n < 2) {
    return false;
  }
  if (n < 4) {
    return true;
  }
  if (n % 2 === 0 || n % 3 === 0) {
    return false;
  }
  for (let divisor = 5; divisor * divisor <= n; divisor += 6) {
    if (n % divisor === 0 || n % (divisor + 2) === 0) {
      return false;
    }
  }
  return true;
};
