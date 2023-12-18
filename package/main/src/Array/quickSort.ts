export const compareFunctionDefault = <T>(a: T, b: T): number =>
  a > b ? 1 : a < b ? -1 : 0;

const medianOfThree = <T>(
  array: T[],
  a: number,
  b: number,
  c: number,
  compareFunction: (a: T, b: T) => number,
): T => {
  const ab = compareFunction(array[a], array[b]);
  const ac = compareFunction(array[a], array[c]);
  const bc = compareFunction(array[b], array[c]);
  if (ab < 0) {
    if (bc < 0) {
      return array[b];
    }
    return ac < 0 ? array[c] : array[a];
  }
  if (ac < 0) {
    return array[a];
  }
  return bc < 0 ? array[c] : array[b];
};

/**
 * 配列を高速にソート
 * @param  {T[]} array 配列
 * @param  {(a: T, b: T) => number} compareFn 比較関数
 * @param  {number} startID 開始インデックス
 * @param  {number} endID 終了インデックス
 * @returns T[]
 * @example quickSort([1, 3, 2, 4, 5], (a, b) => a - b); // [1, 2, 3, 4, 5]
 */
export const quickSort = <T>(
  array: T[],
  compareFunction: (a: T, b: T) => number = compareFunctionDefault<T>,
  startID = 0,
  endID: number = array.length - 1,
  insertionSortThreshold = 10,
): T[] => {
  const partition = (low: number, high: number): number => {
    const pivot = medianOfThree(
      array,
      low,
      Math.floor((low + high) / 2),
      high,
      compareFunction,
    );
    let index = low - 1;
    let index_ = high + 1;
    while (true) {
      do {
        index++;
      } while (compareFunction(array[index], pivot) < 0);
      do {
        index_--;
      } while (compareFunction(array[index_], pivot) > 0);
      if (index >= index_) {
        return index_;
      }
      [array[index], array[index_]] = [array[index_], array[index]];
    }
  };

  const insertionSort = (low: number, high: number) => {
    for (let index = low + 1; index <= high; index++) {
      const key = array[index];
      let index_ = index - 1;
      while (index_ >= low && compareFunction(array[index_], key) > 0) {
        array[index_ + 1] = array[index_];
        index_--;
      }
      array[index_ + 1] = key;
    }
  };

  const sort = (low: number, high: number) => {
    while (low < high) {
      if (high - low < insertionSortThreshold) {
        insertionSort(low, high);
        break;
      }
      const pi = partition(low, high);
      if (pi - low < high - pi) {
        sort(low, pi);
        low = pi + 1;
      } else {
        sort(pi + 1, high);
        high = pi;
      }
    }
  };

  sort(startID, endID);
  return array;
};
