/**
 * 最小値を返す
 * @param {number[]} number_
 * @returns number
 * @example min(1, 2, 3); // 1
 */
export const min = (...number_: number[]) =>
  Reflect.apply(Math.min, null, [...new Set(number_)]);
