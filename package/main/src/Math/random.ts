/**
 * 整数の乱数
 * @param  {number} num
 */
export const random = (num: number) =>
    Math.floor(Math.random() * (num || 1));
