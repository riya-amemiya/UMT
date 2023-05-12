/**
 * x < yになるように入れ替える
 * @param  {number} x
 * @param  {number} y
 */
const ValueSwap = (x: number, y: number) => {
    let tmp: number;
    if (y < x) {
        tmp = y;
        y = x;
        x = tmp;
    }
    return [x, y];
};
export default ValueSwap;
