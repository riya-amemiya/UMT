/**
 * n個からr個を選ぶ順列（nPr）を計算します。
 * @param n - 選ぶ元となる全体の数
 * @param r - 選ぶ個数
 * @returns 順列の結果、または引数が無効な場合はNaN
 */
export const nPr = (n: number, r: number): number => {
  if (n === 0 || r === 0 || n < r) {
    return Number.NaN;
  }
  let result = 1;
  for (let index = 0; index < r; index++) {
    result *= n - index;
  }
  return result;
};
