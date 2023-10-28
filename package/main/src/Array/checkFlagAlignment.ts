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
