import { gcd } from './gcd';
import { nCr } from './nCr';
/**
 * 反復的な思考
 * @param  {number} n
 * @param  {number} r
 * @param  {{x:number;y:number}} p
 * @return {number[]}
 */
export const repeatedTrial = (
    n: number,
    r: number,
    p: { x: number; y: number },
): number[] => {
    let x = nCr(n, r);
    let answer1 = x * Math.pow(p.x, r) * Math.pow(p.y - p.x, n - r);
    let answer2 = Math.pow(p.y, r) * Math.pow(p.y, n - r);
    let greatest_common_divisor = gcd(answer1, answer2);
    return [
        answer1 / greatest_common_divisor,
        answer2 / greatest_common_divisor,
    ];
};
