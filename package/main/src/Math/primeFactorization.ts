/**
 * 素因数分解
 * @param  {number} x
 */
export const primeFactorization = (x: number) => {
  let n = 0;
  let cooyX = x;
  const out: { number: number; count: number }[] = [];
  for (let index = 2; index <= cooyX; index++) {
    if (cooyX % index === 0) {
      n = 0;
      while (cooyX % index === 0) {
        n++;
        cooyX /= index;
      }
      out.push({ number: index, count: n });
    }
  }
  return out;
};
