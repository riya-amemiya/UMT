/**
 * nPr
 * @param  {number} n
 * @param  {number} r
 */
const nPr = (n: number, r: number) => {
    //nPr
    if (n === 0 || r === 0) {
        return NaN;
    }
    let y = n;
    let x = 0;
    while (x === 0) {
        if (r === 1) {
            y *= r;
        }
        r--;
        if (r === 0) {
            x++;
        } else {
            n--;
            if (n === 0) {
                x++;
                break;
            }
            y *= n;
        }
    }
    if (1 > y) return 0;
    return y;
};
export { nPr };
