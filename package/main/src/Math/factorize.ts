/**
 * 因数分解
 * @param  {number} n
 * @returns number[]
 */
export const factorize = (n: number): number[] => {
  const result: number[] = [];
  let copyN = n;
  for (let i = 2; i <= copyN; i++) {
    while (copyN % i === 0) {
      result.push(i);
      copyN /= i;
    }
  }
  return result;
};
