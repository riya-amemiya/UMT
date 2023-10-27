/**
 * 因数分解
 * @param  {number} n
 * @returns number[]
 */
export const factorize = (n: number): number[] => {
  const result: number[] = [];
  let copyN = n;
  for (let index = 2; index <= copyN; index++) {
    while (copyN % index === 0) {
      result.push(index);
      copyN /= index;
    }
  }
  return result;
};
