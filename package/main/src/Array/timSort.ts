import { compareFunctionDefault } from "./compareFunctionDefault";

type CompareFunction<T> = (a: T, b: T) => number;

const MIN_RUN = 32;

const insertionSort = <T>(
  array: T[],
  start: number,
  end: number,
  compareFunction: CompareFunction<T>,
): void => {
  for (let index = start + 1; index <= end; index++) {
    const temporary = array[index];
    let currentIndex = index - 1;
    while (
      currentIndex >= start &&
      compareFunction(array[currentIndex], temporary) > 0
    ) {
      array[currentIndex + 1] = array[currentIndex];
      currentIndex--;
    }
    array[currentIndex + 1] = temporary;
  }
};

const merge = <T>(
  array: T[],
  start: number,
  mid: number,
  end: number,
  compareFunction: CompareFunction<T>,
): void => {
  const left = array.slice(start, mid + 1);
  const right = array.slice(mid + 1, end + 1);
  let leftIndex = 0;
  let rightIndex = 0;
  let arrayIndex = start;

  while (leftIndex < left.length && rightIndex < right.length) {
    if (compareFunction(left[leftIndex], right[rightIndex]) <= 0) {
      array[arrayIndex] = left[leftIndex];
      leftIndex++;
    } else {
      array[arrayIndex] = right[rightIndex];
      rightIndex++;
    }
    arrayIndex++;
  }

  while (leftIndex < left.length) {
    array[arrayIndex] = left[leftIndex];
    leftIndex++;
    arrayIndex++;
  }

  while (rightIndex < right.length) {
    array[arrayIndex] = right[rightIndex];
    rightIndex++;
    arrayIndex++;
  }
};

const getMinRunLength = (input: number): number => {
  let n = input;
  let r = 0;
  while (n >= MIN_RUN) {
    r |= n & 1;
    n >>= 1;
  }
  return n + r;
};

/**
 * TimSortアルゴリズムを実装した関数です。
 * 挿入ソートとマージソートの良い特性を組み合わせたソートアルゴリズムで、
 * 安定なソートを提供し、最悪のケースでもO(n log n)の時間複雑度を持ちます。
 *
 * @param {T[]} array - ソートする配列
 * @param {CompareFunction<T>} [compareFunction=compareFunctionDefault<T>] -
 *        要素比較のための関数
 * @param {number} [start=0] - ソートを開始する配列のインデックス
 * @param {number} [end=array.length - 1] - ソートを終了する配列のインデックス
 * @returns {T[]} - ソートされた配列
 */
export const timSort = <T>(
  array: T[],
  compareFunction: CompareFunction<T> = compareFunctionDefault<T>,
  start = 0,
  end: number = array.length - 1,
): T[] => {
  const n = end - start + 1;
  const minRun = getMinRunLength(n);

  for (let index = start; index <= end; index += minRun) {
    insertionSort(
      array,
      index,
      Math.min(index + MIN_RUN - 1, end),
      compareFunction,
    );
  }

  for (let size = minRun; size < n; size *= 2) {
    for (let left = start; left <= end; left += 2 * size) {
      const mid = left + size - 1;
      const right = Math.min(left + 2 * size - 1, end);

      if (mid < right) {
        merge(array, left, mid, right, compareFunction);
      }
    }
  }

  return array;
};
