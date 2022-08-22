/**
 * 最小値を返す
 * @param  {number[]} num
 */
const min = (num: number[]) =>
    Math.min.apply(null, [...new Set(num)]);
export default min;
