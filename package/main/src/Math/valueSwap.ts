/**
 * x < yになるように入れ替える
 * @param  {number} x
 * @param  {number} y
 * @return {number[]}
 */
export const valueSwap = (x: number, y: number): number[] => {
    let tmp: number;
    if (y < x) {
        tmp = y;
        y = x;
        x = tmp;
    }
    return [x, y];
};
