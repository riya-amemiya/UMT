/**
 * フラグが揃っているかどうかをチェックする
 * @param matrix フラグを持つセルの二次元配列
 * @returns フラグが揃っているかどうか
 * @example checkFlagAlignment([
 *  [{ value: 1, flag: true }, { value: 2, flag: false }, { value: 3, flag: true }],
 *  [{ value: 4, flag: false }, { value: 5, flag: true }, { value: 6, flag: false }],
 *  [{ value: 7, flag: false }, { value: 8, flag: true }, { value: 9, flag: true }],
 * ]); // true
 */
export const checkFlagAlignment = <T extends { flag: boolean }>(
  matrix: T[][],
): boolean => {
  const rows = matrix.length;
  const cols = matrix[0].length;

  // 横方向のチェック
  for (let index = 0; index < rows; index++) {
    if (matrix[index].every((cell) => cell.flag)) {
      return true;
    }
  }

  // 縦方向のチェック
  for (let index = 0; index < cols; index++) {
    if (matrix.every((row) => row[index].flag)) {
      return true;
    }
  }

  // 斜め方向のチェック (左上から右下へ)
  if (matrix.every((row, index) => row[index].flag)) {
    return true;
  }

  // 斜め方向のチェック (左下から右上へ)
  if (matrix.every((row, index) => row[cols - index - 1].flag)) {
    return true;
  }

  return false;
};
