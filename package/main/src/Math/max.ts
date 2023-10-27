/**
 * 最大値を返す
 * @param  {number[]} num
 */
export const max = (...number_: number[]) =>
  Reflect.apply(Math.max, null, [...new Set(number_)]);
