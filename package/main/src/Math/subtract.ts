import { getDecimalLength } from "./getDecimalLength";
import { max } from "./max";
import { multiplication } from "./multiplication";

/**
 * 誤差のない引き算
 * @param  {number} x
 * @param  {number} y
 */

export const subtract = (x: number, y: number): number => {
<<<<<<< HEAD
  // 10の何乗かを取得
  const z = 10 ** max(getDecimalLength(x), getDecimalLength(y));
  // 小数点を揃えてから引き算
  return (multiplication(x, z) - multiplication(y, z)) / z;
=======
	// 10の何乗かを取得
	const z = 10 ** max(getDecimalLength(x), getDecimalLength(y));
	// 小数点を揃えてから引き算
	return (multiplication(x, z) - multiplication(y, z)) / z;
>>>>>>> 58a3e53 (修正)
};
