import division from './division';

/**
 * 素数判定
 * @param  {number} n
 */
const isPrimeNumber = (n: number) => {
    if (n < 2) return false;
    else if (n == 2) return true;
    else if (n % 2 == 0) return false;
    for (let i = 3; i <= Math.sqrt(n); i++) {
        if (division(n, i)[1] == 0) {
            return false;
        }
    }
    return true;
};
export default isPrimeNumber;
