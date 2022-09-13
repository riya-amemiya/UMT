import getDecimalLength from './getDecimalLength';
/**
 * 誤差のない割り算
 * @param  {number} x
 * @param  {number} y
 * @param  {boolean} [isFloor=true]
 */
interface DIVISION {
    (x: number, y: number, isFloor?: true): number;
    (x: number, y: number, isFloor?: false): number[];
}
const division = ((x: number, y: number, isFloor: boolean = true) => {
    const n = 10 ** (getDecimalLength(x) + getDecimalLength(y));
    x = +(x + '').replace('.', '');
    y = +(y + '').replace('.', '');
    return isFloor ? x / y / n : [(x - (x % y)) / y / n, (x % y) / n];
}) as DIVISION;
export default division;
