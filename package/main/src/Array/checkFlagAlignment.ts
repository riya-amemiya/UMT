/**
 * Check if flags are aligned in any direction (horizontal, vertical, or diagonal)
 * @param matrix Two-dimensional array of cells containing flags
 * @returns True if flags are aligned in any direction, false otherwise
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

  // Check horizontal alignment
  for (let index = 0; index < rows; index++) {
    if (matrix[index].every((cell) => cell.flag)) {
      return true;
    }
  }

  // Check vertical alignment
  for (let index = 0; index < cols; index++) {
    if (matrix.every((row) => row[index].flag)) {
      return true;
    }
  }

  // Check diagonal alignment (top-left to bottom-right)
  if (matrix.every((row, index) => row[index].flag)) {
    return true;
  }

  // Check diagonal alignment (bottom-left to top-right)
  if (matrix.every((row, index) => row[cols - index - 1].flag)) {
    return true;
  }

  return false;
};
