import { compareFunctionDefault } from "./compareFunctionDefault";

/**
 * TimSortアルゴリズムを実装した関数です。
 * 挿入ソートとマージソートの良い特性を組み合わせたソートアルゴリズムで、
 * 安定なソートを提供し、最悪のケースでもO(n log n)の時間複雑度を持ちます。
 *
 * @param {T[]} array - ソートする配列
 * @param {(a: T, b: T) => number} [compareFunction=compareFunctionDefault<T>] -
 *        要素比較のための関数
 * @param {number} [start=0] - ソートを開始する配列のインデックス
 * @param {number} [end=array.length - 1] - ソートを終了する配列のインデックス
 * @returns {T[]} - ソートされた配列
 */
export const timSort = <T>(
  array: T[],
  compareFunction: (a: T, b: T) => number = compareFunctionDefault<T>,
  start = 0,
  end: number = array.length - 1,
): T[] => {
  const MIN_RUN = 32;

  const insertionSort = (start: number, end: number) => {
    for (let index = start; index <= end; index++) {
      const temporary = array[index];
      let index_ = index - 1;
      while (index_ >= start && compareFunction(array[index_], temporary) > 0) {
        array[index_ + 1] = array[index_];
        index_--;
      }
      array[index_ + 1] = temporary;
    }
  };

  const merge = (start: number, mid: number, end: number) => {
    const left = array.slice(start, mid + 1);
    const right = array.slice(mid + 1, end + 1);
    let index = 0;
    let index_ = 0;
    let k = start;
    while (index < left.length && index_ < right.length) {
      if (compareFunction(left[index], right[index_]) <= 0) {
        array[k] = left[index];
        index++;
      } else {
        array[k] = right[index_];
        index_++;
      }
      k++;
    }
    while (index < left.length) {
      array[k] = left[index];
      index++;
      k++;
    }
    while (index_ < right.length) {
      array[k] = right[index_];
      index_++;
      k++;
    }
  };

  const getMinRunLength = (n: number) => {
    let r = 0;
    while (n >= MIN_RUN) {
      r |= n & 1;
      n >>= 1;
    }
    return n + r;
  };

  const n = end - start + 1;
  const minRun = getMinRunLength(n);

  for (let index = start; index <= end; index += minRun) {
    insertionSort(index, Math.min(index + MIN_RUN - 1, end));
  }

  for (let size = minRun; size < n; size *= 2) {
    for (let left = start; left <= end; left += 2 * size) {
      const mid = left + size - 1;
      const right = Math.min(left + 2 * size - 1, end);

      if (mid < right) {
        merge(left, mid, right);
      }
    }
  }

  return array;
};
