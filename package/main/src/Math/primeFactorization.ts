/**
 * 素因数分解
 * @param  {number} x
 */
export const primeFactorization = (x: number) => {
  let n = 0;
  let cooyX = x;
  const out: { number: number; count: number }[] = [];
  for (let i = 2; i <= cooyX; i++) {
    if (cooyX % i === 0) {
      n = 0;
      while (cooyX % i === 0) {
        n++;
        cooyX /= i;
      }
      out.push({ number: i, count: n });
    }
  }
  return out;
};
