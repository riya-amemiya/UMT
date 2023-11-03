import { random } from "../Math/random";
/**
 * 配列を高速にソート
 * @param  {unknown[]} array 配列
 * @param  {number} startID 開始インデックス
 * @param  {number} endID 終了インデックス
 * @returns unknown[]
 * @example quickSort([1, 3, 2, 4, 5]); // [1, 2, 3, 4, 5]
 */

export const quickSort = <A extends unknown[]>(
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  array: any[],
  startID = 0,
  endID: number = array.length - 1,
): A => {
  const pivot = array[random(endID, startID)];
  let left = startID;
  let right = endID;
  while (true) {
    while (array[left] < pivot) {
      left++;
    }
    while (pivot < array[right]) {
      right--;
    }
    if (right <= left) {
      break;
    }
    const temporary = array[left];
    array[left] = array[right];
    array[right] = temporary;
    left++;
    right--;
  }
  if (startID < left - 1) {
    quickSort(array, startID, left - 1);
  }
  if (right + 1 < endID) {
    quickSort(array, right + 1, endID);
  }
  return array as A;
};
