/**
 * 因数分解
 * @param  {number} n
 */
const factorize = (n: number): number[] => {
    const result: number[] = [];
    for (let i = 2; i <= n; i++) {
        while (n % i === 0) {
            result.push(i);
            n /= i;
        }
    }
    return result;
};
export { factorize };
