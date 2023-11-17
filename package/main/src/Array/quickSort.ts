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
  if (startID >= endID) {
    return array as A;
  }

  // 中央値をピボットとして選択
  const pivotIndex = Math.floor((startID + endID) / 2);
  const pivot = array[pivotIndex];
  let left = startID;
  let right = endID;

  while (left <= right) {
    while (array[left] < pivot) {
      left++;
    }
    while (array[right] > pivot) {
      right--;
    }
    if (left <= right) {
      [array[left], array[right]] = [array[right], array[left]];
      left++;
      right--;
    }
  }

  // より小さいサブ配列を先にソート
  if (startID < right) {
    quickSort(array, startID, right);
  }
  if (left < endID) {
    quickSort(array, left, endID);
  }

  return array as A;
};
