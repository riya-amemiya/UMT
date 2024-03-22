import { compareFunctionDefault } from "./compareFunctionDefault";

/**
 * マージソート
 * @param  {T[]} array 配列
 * @param  {(a: T, b: T) => number} compareFunction 比較関数
 * @returns T[]
 * @example mergeSort([1, 3, 2, 4, 5], (a, b) => a - b); // [1, 2, 3, 4, 5]
 */
export const mergeSort = <T>(
  array: T[],
  compareFunction: (a: T, b: T) => number = compareFunctionDefault,
): T[] => {
  if (array.length <= 1) {
    return array;
  }

  const middle = Math.floor(array.length / 2);
  const left = array.slice(0, middle);
  const right = array.slice(middle);

  return merge(
    mergeSort(left, compareFunction),
    mergeSort(right, compareFunction),
    compareFunction,
  );
};

/**
 * 2つの配列をマージする関数
 * @param left - マージする左の配列
 * @param right - マージする右の配列
 * @param compareFunction - 比較関数
 * @returns マージされた新しい配列
 */
function merge<T>(
  left: T[],
  right: T[],
  compareFunction: (a: T, b: T) => number,
): T[] {
  const array: T[] = [];
  let lIndex = 0;
  let rIndex = 0;

  while (lIndex < left.length && rIndex < right.length) {
    if (compareFunction(left[lIndex], right[rIndex]) <= 0) {
      array.push(left[lIndex++]);
    } else {
      array.push(right[rIndex++]);
    }
  }

  return array.concat(left.slice(lIndex)).concat(right.slice(rIndex));
}
