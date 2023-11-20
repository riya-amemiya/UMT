/**
 * 配列を高速にソート
 * @param  {unknown[]} array 配列
 * @param  {number} startID 開始インデックス
 * @param  {number} endID 終了インデックス
 * @returns unknown[]
 * @example quickSort([1, 3, 2, 4, 5]); // [1, 2, 3, 4, 5]
 */
export const quickSort = <T>(
  array: T[],
  startID = 0,
  endID: number = array.length - 1,
): T[] => {
  const swap = (index: number, index_: number) => {
    [array[index], array[index_]] = [array[index_], array[index]];
  };

  const partition = (low: number, high: number): number => {
    // median-of-threeルールを使用してピボットを選択
    const mid = Math.floor((low + high) / 2);
    const pivot = array[mid];
    swap(mid, high);
    let index = low;
    for (let index_ = low; index_ < high; index_++) {
      if (array[index_] < pivot) {
        swap(index, index_);
        index++;
      }
    }
    swap(index, high);
    return index;
  };

  const sort = (low: number, high: number) => {
    let copyLow = low;
    let copyHigh = high;
    while (copyLow < copyHigh) {
      const pi = partition(copyLow, copyHigh);
      if (pi - copyLow < copyHigh - pi) {
        sort(copyLow, pi - 1);
        copyLow = pi + 1;
      } else {
        sort(pi + 1, copyHigh);
        copyHigh = pi - 1;
      }
    }
  };

  sort(startID, endID);
  return array;
};
