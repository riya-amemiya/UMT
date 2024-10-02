/**
 * 共通の要素を取り出す
 * @param {T[]} array 最初の配列
 * @param {T[][]} arrays 他の配列群
 * @returns {O} 共通の要素を含む配列
 * @example getArraysCommon([1, 2, 3], [2, 3, 4], [2, 5, 3]); // [2, 3]
 */
export const getArraysCommon = <O, T = unknown>(
  array: T[],
  ...arrays: T[][]
): O =>
  [
    ...arrays.reduce(
      (accumulator, current) => accumulator.intersection(new Set(current)),
      new Set(array),
    ),
  ] as O;
