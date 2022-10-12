import { getDecimalLength } from './getDecimalLength';
import { max } from './max';
import { multiplication } from './multiplication';

/**
 * 誤差のない引き算
 * @param  {number} x
 * @param  {number} y
 */

export const subtract = (x: number, y: number) => {
    const z = Math.pow(
        10,
        max(getDecimalLength(x), getDecimalLength(y)),
    );
    return (
        (multiplication(x, z) -
            [multiplication](#multiplication)(y, z)) /
        z
    );
};
