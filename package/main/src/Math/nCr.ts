import { nPr } from "./nPr";

/**
 * n個からr個を選ぶ組み合わせ（nCr）を計算します。
 * @param n - 選ぶ元となる全体の数
 * @param r - 選ぶ個数
 * @returns 組み合わせの結果、または引数が無効な場合はNaN
 */
export const nCr = (n: number, r: number): number => {
  if (n === 0 || r === 0 || n < r) {
    return Number.NaN;
  }

  const numerator = nPr(n, r);
  let denominator = 1;

  for (let index = 2; index <= r; index++) {
    denominator *= index;
  }

  const result = numerator / denominator;

  return result;
};
