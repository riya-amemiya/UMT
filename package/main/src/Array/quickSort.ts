import { compareFunctionDefault } from "./compareFunctionDefault";

type CompareFunction<T> = (a: T, b: T) => number;

const medianOfThree = <T>(
  array: T[],
  a: number,
  b: number,
  c: number,
  compareFunction: CompareFunction<T>,
): T => {
  const ab = compareFunction(array[a], array[b]);
  if (ab < 0) {
    const bc = compareFunction(array[b], array[c]);
    if (bc < 0) {
      return array[b];
    }
    if (compareFunction(array[a], array[c]) < 0) {
      return array[c];
    }
    return array[a];
  }
  const ac = compareFunction(array[a], array[c]);
  if (ac < 0) {
    return array[a];
  }
  if (compareFunction(array[b], array[c]) < 0) {
    return array[c];
  }
  return array[b];
};

const partition = <T>(
  array: T[],
  low: number,
  high: number,
  compareFunction: CompareFunction<T>,
): number => {
  const pivot = medianOfThree(
    array,
    low,
    Math.floor((low + high) / 2),
    high,
    compareFunction,
  );
  let left = low - 1;
  let right = high + 1;

  while (true) {
    do {
      left++;
    } while (compareFunction(array[left], pivot) < 0);
    do {
      right--;
    } while (compareFunction(array[right], pivot) > 0);

    if (left >= right) {
      return right;
    }

    [array[left], array[right]] = [array[right], array[left]];
  }
};

const insertionSort = <T>(
  array: T[],
  low: number,
  high: number,
  compareFunction: CompareFunction<T>,
): void => {
  for (let index = low + 1; index <= high; index++) {
    const key = array[index];
    let currentIndex = index - 1;
    while (
      currentIndex >= low &&
      compareFunction(array[currentIndex], key) > 0
    ) {
      array[currentIndex + 1] = array[currentIndex];
      currentIndex--;
    }
    array[currentIndex + 1] = key;
  }
};

const sortImpl = <T>(
  array: T[],
  lowInit: number,
  highInit: number,
  compareFunction: CompareFunction<T>,
  insertionSortThreshold: number,
): void => {
  let low = lowInit;
  let high = highInit;

  while (low < high) {
    if (high - low < insertionSortThreshold) {
      insertionSort(array, low, high, compareFunction);
      return;
    }
    const pivotIndex = partition(array, low, high, compareFunction);
    if (pivotIndex - low < high - pivotIndex) {
      sortImpl(array, low, pivotIndex, compareFunction, insertionSortThreshold);
      low = pivotIndex + 1;
    } else {
      sortImpl(
        array,
        pivotIndex + 1,
        high,
        compareFunction,
        insertionSortThreshold,
      );
      high = pivotIndex;
    }
  }
};

/**
 * 配列を高速にソート
 * QuickSortとInsertionSortを組み合わせたアルゴリズムで配列を高速にソートします
 * @param {T[]} array 配列
 * @param {CompareFunction<T>} compareFunction 比較関数
 * @param {number} startIndex 開始インデックス
 * @param {number} endIndex 終了インデックス
 * @param {number} insertionSortThreshold 挿入ソートの閾値
 * @returns T[]
 * @example quickSort([1, 3, 2, 4, 5], (a, b) => a - b); // [1, 2, 3, 4, 5]
 */
export const quickSort = <T>(
  array: T[],
  compareFunction: CompareFunction<T> = compareFunctionDefault<T>,
  startIndex = 0,
  endIndex = array.length - 1,
  insertionSortThreshold = 10,
): T[] => {
  // 有効な範囲内でのみソートを行う
  const validStartIndex = Math.max(0, Math.min(startIndex, array.length - 1));
  const validEndIndex = Math.max(
    validStartIndex,
    Math.min(endIndex, array.length - 1),
  );

  sortImpl(
    array,
    validStartIndex,
    validEndIndex,
    compareFunction,
    insertionSortThreshold,
  );
  return array;
};
