/**
 * 素数判定
 * @param  {number} n
 * @returns boolean
 * @example isPrimeNumber(2); // true
 */
export const isPrimeNumber = (n: number): boolean => {
  if (n <= 1) {
    return false;
  }

  for (let index = 2; index <= Math.sqrt(n); index++) {
    if (n % index === 0) {
      return false;
    }
  }

  return true;
};
