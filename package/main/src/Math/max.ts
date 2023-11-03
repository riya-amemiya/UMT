/**
 * 最大値を返す
 * @param  {number[]} num
 * @returns number
 * @example max(1, 2, 3); // 3
 */
export const max = (...number_: number[]) =>
  Reflect.apply(Math.max, null, [...new Set(number_)]);
