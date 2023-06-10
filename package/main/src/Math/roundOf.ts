/**
 * 四捨五入
 * @param  {number} num - 四捨五入する数値
 * @param  {number} precision - 四捨五入する桁数
 * @returns number
 */
export const roundOf = (num: number, precision: number) => {
<<<<<<< HEAD
  return Math.round(num * 10 ** precision) / 10 ** precision;
=======
	return Math.round(num * 10 ** precision) / 10 ** precision;
>>>>>>> 58a3e53 (修正)
};
