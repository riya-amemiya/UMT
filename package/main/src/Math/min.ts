/**
 * 最小値を返す
 * @param  {number[]} num
 */
export const min = (...number_: number[]) =>
  Reflect.apply(Math.min, null, [...new Set(number_)]);
