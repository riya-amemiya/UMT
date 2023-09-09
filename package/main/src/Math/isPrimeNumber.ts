/**
 * 素数判定
 * @param  {number} n
 * @returns boolean
 */
export const isPrimeNumber = (n: number): boolean => {
  if (n <= 1) {
    return false;
  }

  for (let i = 2; i <= Math.sqrt(n); i++) {
    if (n % i === 0) {
      return false;
    }
  }

  return true;
};
