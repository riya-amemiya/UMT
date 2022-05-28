/**
 * 最大値を返す
 * @param  {number[]} num
 */
const max = (num: number[]) =>
    Math.max.apply(null, [...new Set(num)]);
export default max;
