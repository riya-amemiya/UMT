/**
 * 整数の乱数
 * @param  {number} max 最大値
 * @param  {number} min 最小値
 * @return {number}
 */
export const random = (max: number, min = 0): number =>
  Math.floor(Math.random() * (max - min + 1)) + min;
