import gcd from './gcd';
/**
 * 約分
 * @param  {number} x
 * @param  {number} y
 */
const reduce = (x: number, y: number) => {
    if (x === 0 || y === 0) {
        return { x: NaN, y: NaN };
    }
    let n = gcd(x, y);
    return { x: x / n, y: y / n, gcd: gcd(x, y) };
};
export default reduce;
