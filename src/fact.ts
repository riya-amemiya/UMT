/**
 * 階乗
 * @param  {number} x
 */
const fact = (x: number): number => {
    if (x === 0) {
        return 1;
    } else {
        return x * fact(x - 1);
    }
};
export default fact;
