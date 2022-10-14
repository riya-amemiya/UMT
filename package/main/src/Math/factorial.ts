/**
 * 階乗
 * @param  {number} x
 * @returns number
 */
export const factorial = (x: number): number => {
    let result = 1;
    if (x !== 0) {
        while (x > 1) {
            result *= x;
            x--;
        }
    }
    return result;
};
