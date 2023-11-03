import { nCr } from "./nCr";

/**
 * n個からr個を重複を許して選ぶ組み合わせ（nHr）を計算します。
 * @param n - 選ぶ元となる全体の数
 * @param r - 選ぶ個数
 * @returns 重複組み合わせの結果、または引数が無効な場合はNaN
 * @example nHr(5, 2); // 15
 */
export const nHr = (n: number, r: number): number => {
  if (n === 0 || r === 0 || n < 0 || r < 0) {
    return Number.NaN;
  }

  const result = nCr(n + r - 1, r);

  return result;
};
