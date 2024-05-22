/**
 * 配列から指定された数の要素をランダムに選択する
 * @param array 元の配列
 * @param count 選択する要素の数
 * @param allowDuplicates 重複を許すかどうか (デフォルトは false)
 * @returns ランダムに選択された要素の配列
 * @example randomSelect([1, 2, 3, 4, 5], 2); // [3, 1]
 */
export const randomSelect = <T>(
  array: T[],
  count: number,
  allowDuplicates = false,
): T[] => {
  const result: T[] = [];
  const usedIndices = new Set<number>();

  while (
    result.length < count &&
    (allowDuplicates || result.length < array.length)
  ) {
    const randomIndex = Math.floor(Math.random() * array.length);
    if (allowDuplicates || !usedIndices.has(randomIndex)) {
      usedIndices.add(randomIndex);
      result.push(array[randomIndex]);
    }
  }

  return result;
};
