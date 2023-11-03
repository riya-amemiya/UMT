import { addition } from "@/Math/addition";
/**
 * 配列の合計を返す
 * @param {number[]} x 数値型の配列
 * @returns 配列の合計
 * @example sum([1, 2, 3]); // 6
 */
export const sum = (x: number[]) => {
  return x.reduce((a, b) => addition(a, b));
};
