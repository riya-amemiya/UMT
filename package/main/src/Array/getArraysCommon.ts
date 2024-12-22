/**
 * 共通の要素を取り出す
 * @param {T[]} array 最初の配列
 * @param {T[][]} arrays 他の配列群
 * @returns {O} 共通の要素を含む配列
 * @example getArraysCommon([1, 2, 3], [2, 3, 4], [2, 5, 3]); // [2, 3]
 */
export const getArraysCommon = <O, T extends unknown[] = unknown[]>(
  array: T,
  ...arrays: T[]
): O => {
  const result = [array, ...arrays].reduce((previous, current) => {
    return previous.filter((item: unknown) => current.includes(item)) as T;
  });
  return result as unknown as O;
};
