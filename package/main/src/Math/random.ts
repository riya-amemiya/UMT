/**
 * 整数の乱数
 * @param  {number} num
 * @param  {number} min 最小値
 * @return {number}
 */
export const random = (num: number, min: number = 0): number =>
    Math.floor(Math.random() * ((num || 1) - min) + min);
