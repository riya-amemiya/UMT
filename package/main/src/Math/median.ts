import { quickSort } from "@/Array/quickSort";

/**
 * 数値配列の中央値を計算する
 * @param {number[]} array 数値型の配列
 * @returns 中央値
 * @example median([1, 3, 3, 6, 7, 8, 9]); // 6
 */
export const median = (array: number[]): number => {
  const sortedArray: number[] = quickSort(array);
  const mid = Math.floor(sortedArray.length / 2);

  return sortedArray.length % 2 === 0
    ? (sortedArray[mid - 1] + sortedArray[mid]) / 2
    : sortedArray[mid];
};
