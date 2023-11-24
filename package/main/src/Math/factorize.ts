/**
 * 因数分解
 * @param  {number} n
 * @returns number[]
 * @example factorize(12); // [2, 2, 3]
 */
export const factorize = (n: number): number[] => {
  const result: number[] = [];
  for (let index = 2; index <= n; index++) {
    while (n % index === 0) {
      result.push(index);
      n /= index;
    }
  }
  return result;
};
