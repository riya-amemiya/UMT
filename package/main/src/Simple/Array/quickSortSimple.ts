import { compareFunctionDefault } from "@/Array/compareFunctionDefault";
import { quickSort } from "@/Array/quickSort";

/**
 * 配列を高速にソート
 * @param {T[]} array 配列
 * @param {(a: T, b: T) => number} compareFunction 比較関数
 * @param {number} startID 開始インデックス
 * @param {number} endID 終了インデックス
 * @returns T[]
 * @example quickSort([1, 3, 2, 4, 5], (a, b) => a - b); // [1, 2, 3, 4, 5]
 */
export const quickSortSimple = <T>(
  array: T[],
  compareFunction: (a: T, b: T) => number = compareFunctionDefault<T>,
  startID = 0,
  endID: number = array.length - 1,
): T[] => {
  let localStartID = startID;
  let localEndID = endID;
  if (
    localStartID < 0 ||
    localStartID >= array.length ||
    localStartID > localEndID
  ) {
    localStartID = 0;
  }
  if (localEndID < 0 || localEndID >= array.length) {
    localEndID = array.length - 1;
  }
  return quickSort(array, compareFunction, localStartID, localEndID);
};
