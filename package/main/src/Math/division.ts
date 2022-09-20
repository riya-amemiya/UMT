/**
 * 誤差のない割り算
 * @param  {number} x
 * @param  {number} y
 * @param  {boolean} [isFloor=true]
 */
export interface DIVISION {
    (x: number, y: number, isFloor?: true): number;
    (x: number, y: number, isFloor?: false): number[];
}
const division = ((x: number, y: number, isFloor: boolean = true) => {
    x = +(x + '').replace('.', '');
    y = +(y + '').replace('.', '');
    return isFloor ? x / y : [(x - (x % y)) / y, x % y];
}) as DIVISION;
export default division;
