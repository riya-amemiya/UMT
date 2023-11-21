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
  const partition = (low: number, high: number): number => {
    // median-of-threeルールを使用してピボットを選択
    const mid = Math.floor((low + high) / 2);
    const pivot = array[mid];
    [array[high], array[mid]] = [array[mid], array[high]];
    let index = low;
    for (let index_ = low; index_ < high; index_++) {
      if (array[index_] < pivot) {
        [array[index], array[index_]] = [array[index_], array[index]];
        index++;
      }
    }
    [array[index], array[high]] = [array[high], array[index]];
    return index;
  };

  const insertionSort = (low: number, high: number) => {
    for (let index = low + 1; index <= high; index++) {
      const key = array[index];
      let index_ = index - 1;
      while (index_ >= low && array[index_] > key) {
        array[index_ + 1] = array[index_];
        index_--;
      }
      array[index_ + 1] = key;
    }
  };

  const sort = (low: number, high: number) => {
    while (low < high) {
      // 小さい配列に対しては挿入ソートを使用
      if (high - low < 10) {
        insertionSort(low, high);
        break;
      }
      const pi = partition(low, high);
      if (pi - low < high - pi) {
        sort(low, pi - 1);
        low = pi + 1;
      } else {
        sort(pi + 1, high);
        high = pi - 1;
      }
    }
  };

  sort(startID, endID);
  return array;
};
