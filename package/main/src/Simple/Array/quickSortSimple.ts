import { compareFunctionDefault } from "@/Array/compareFunctionDefault";
import { quickSort } from "@/Array/quickSort";

/**
 * 配列を高速にソート
 * @param  {T[]} array 配列
 * @param  {(a: T, b: T) => number} compareFn 比較関数
 * @param  {number} startID 開始インデックス
 * @param  {number} endID 終了インデックス
 * @returns T[]
 * @example quickSort([1, 3, 2, 4, 5], (a, b) => a - b); // [1, 2, 3, 4, 5]
 */
export const quickSortSimple = <T>(
  array: T[],
  compareFunction: (a: T, b: T) => number = compareFunctionDefault<T>,
  startID = 0,
  endID: number = array.length - 1,
): T[] => {
  if (startID < 0 || startID >= array.length || startID > endID) {
    startID = 0;
  }
  if (endID < 0 || endID >= array.length) {
    endID = array.length - 1;
  }
  return quickSort(array, compareFunction, startID, endID);
};
