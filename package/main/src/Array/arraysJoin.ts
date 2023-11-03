/**
 * 重複をしないで結合
 * @param  {unknown[]} array
 * @param  {unknown[]} ...arrays
 * @returns unknown[]
 * @example arraysJoin([1, 2, 3], [2, 3, 4]); // [1, 2, 3, 4]
 */
export const arraysJoin = <A extends unknown[]>(
  array: unknown[],
  ...arrays: unknown[]
) => {
  return [...new Set(array.concat(...arrays))] as A;
};
