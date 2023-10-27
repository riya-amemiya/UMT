/**
 * 四捨五入
 * @param  {number} num - 四捨五入する数値
 * @param  {number} precision - 四捨五入する桁数
 * @returns number
 */
export const roundOf = (number_: number, precision: number) => {
  return Math.round(number_ * 10 ** precision) / 10 ** precision;
};
