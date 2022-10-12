import { getDecimalLength } from './getDecimalLength';

/**
 * 誤差のない掛け算
 * @param  {number} x
 * @param  {number} y
 */
const multiplication = (x: number, y: number) => {
    const n = 10 ** (getDecimalLength(x) + getDecimalLength(y));
    x = +(x + '').replace('.', '');
    y = +(y + '').replace('.', '');
    return (x * y) / n;
};
export { multiplication };
