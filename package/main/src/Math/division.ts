import getDecimalLength from './getDecimalLength';
/**
 * 誤差のない割り算
 * @param  {number} x
 * @param  {number} y
 */
const division = (x: number, y: number) => {
    const n = 10 ** (getDecimalLength(x) + getDecimalLength(y));
    x = +(x + '').replace('.', '');
    y = +(y + '').replace('.', '');
    return [x / y / n, (x % y) / n];
};
export default division;
