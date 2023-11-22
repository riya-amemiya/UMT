import { compareFunctionDefault, quickSort } from "@/Array/quickSort";

/**
 * 配列を非同期に高速にソート
 * @param  {unknown[]} array 配列
 * @param  {(a: T, b: T) => number} compareFn 比較関数
 * @param  {number} startID 開始インデックス
 * @param  {number} endID 終了インデックス
 * @returns Promise<unknown[]>
 * @example quickSortAsync([1, 3, 2, 4, 5]); // Promise<[1, 2, 3, 4, 5]>
 */
export const quickSortAsync = async <T>(
  array: T[],
  compareFunction: (a: T, b: T) => number = compareFunctionDefault<T>,
  startID = 0,
  endID: number = array.length - 1,
): Promise<T[]> => {
  return new Promise((resolve) => {
    resolve(quickSort(array, compareFunction, startID, endID));
  });
};
