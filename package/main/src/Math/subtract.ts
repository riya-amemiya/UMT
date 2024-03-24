import { getDecimalLength } from "./getDecimalLength";
import { max } from "./max";
import { multiplication } from "./multiplication";

/**
 * 誤差のない引き算を任意長の引数で行う
 * @param  {number[]} numbers 引き算を行う数値の配列
 * @returns number
 * @example subtract([0.1, 0.2, 0.3]); // -0.4
 */

export const subtract = (...numbers: number[]): number => {
  return numbers.reduce((accumulator, current, index) => {
    if (index === 0) {
      return current;
    }
    // 10の何乗かを取得
    const z =
      10 ** max(getDecimalLength(accumulator), getDecimalLength(current));
    // 小数点を揃えてから引き算
    return (multiplication(accumulator, z) - multiplication(current, z)) / z;
  }, 0);
};
