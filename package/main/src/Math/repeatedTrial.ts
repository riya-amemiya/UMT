import { gcd } from "./gcd";
import { nCr } from "./nCr";
/**
 * 反復的な思考
 * @param  {number} n
 * @param  {number} r
 * @param  {{x:number;y:number}} p
 * @return {number[]}
 * @example repeatedTrial(5, 2, {x: 1/3, y: 2/3}); // [10, 27]
 */
export const repeatedTrial = (
  n: number,
  r: number,
  p: { x: number; y: number },
): number[] => {
  const x = nCr(n, r);
  const answer1 = x * p.x ** r * (p.y - p.x) ** (n - r);
  const answer2 = p.y ** r * p.y ** (n - r);
  const greatest_common_divisor = gcd(answer1, answer2);
  return [answer1 / greatest_common_divisor, answer2 / greatest_common_divisor];
};
