/**
 * 最大値を返す
 * @param  {number[]} num
 */
export const max = (...num: number[]) =>
	Math.max.apply(null, [...new Set(num)]);
