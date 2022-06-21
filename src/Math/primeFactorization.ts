/**
 * 素因数分解
 * @param  {number} x
 */
const primeFactorization = (x: number) => {
    let n = 0;
    const out: { number: number; count: number }[] = [];
    for (let i = 2; i <= x; i++) {
        if (x % i === 0) {
            n = 0;
            while (x % i === 0) {
                n++;
                x /= i;
            }
            out.push({ number: i, count: n });
        }
    }
    return out;
};
export default primeFactorization;
