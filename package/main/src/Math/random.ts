/**
 * 整数の乱数
 * @param  {number} max 最大値
 * @param  {number} min 最小値
 * @return {number}
 * @example random(10); // 0~10
 */
export const random = (max: number, min = 0): number =>
  Math.floor(Math.random() * (max - min + 1)) + min;
