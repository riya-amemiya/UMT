/**
 * 階乗
 * @param  {number} x
 */
const factorial = (x: number): number => {
    if (x === 0) {
        return 1;
    }
    return x * factorial(x - 1);
};
export default factorial;
