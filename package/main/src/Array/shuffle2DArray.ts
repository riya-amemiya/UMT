import { shuffle } from "@/Array/shuffle";

/**
 * 2次元配列の各サブ配列と全体の配列をシャッフルします。
 * @param array シャッフルする2次元配列
 * @returns シャッフルされた2次元配列
 * @example
 * shuffle2DArray([[1, 2], [3, 4], [5, 6]]);
 * // 例: [[4, 3], [1, 2], [6, 5]]
 */
export const shuffle2DArray = <T>(array: T[][]): T[][] => {
  // 各サブ配列をシャッフル
  const shuffledSubArrays = array.map((subArray) => shuffle(subArray));
  // 全体の配列をシャッフル
  return shuffle(shuffledSubArrays);
};
