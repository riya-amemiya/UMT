/**
 * 2次元配列全体の配列をシャッフルします。
 * @param array シャッフルする2次元配列
 * @returns シャッフルされた2次元配列
 * @example
 * shuffle2DArray([[1, 2], [3, 4], [5, 6]]);
 * // 例: [[1, 3], [6, 4], [2, 5]]
 */
export const shuffle2DArray = <T>(array: T[][]): T[][] => {
  // 2次元配列の要素を1次元配列に平坦化し、シャッフルする
  const flatArray: T[] = [];
  for (const subArray of array) {
    flatArray.push(...subArray);
  }
  for (let index = flatArray.length - 1; index > 0; index--) {
    const index_ = Math.floor(Math.random() * (index + 1));
    [flatArray[index], flatArray[index_]] = [
      flatArray[index_],
      flatArray[index],
    ];
  }

  // シャッフルされた1次元配列を2次元配列に再構成する
  let rowIndex = 0;
  return array.map((subArray) => {
    const newRow = flatArray.slice(rowIndex, rowIndex + subArray.length);
    rowIndex += subArray.length;
    return newRow;
  });
};
