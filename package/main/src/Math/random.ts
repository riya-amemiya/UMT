/**
 * 整数の乱数
 * @param  {number} num
 */
const random = (num: number) =>
    Math.floor(Math.random() * (num || 1));
export default random;
