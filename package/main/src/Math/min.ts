/**
 * 最小値を返す
 * @param  {number[]} num
 */
export const min = (...num: number[]) =>
	Math.min.apply(null, [...new Set(num)]);
