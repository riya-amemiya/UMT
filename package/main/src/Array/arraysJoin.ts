/**
 * 重複をしないで結合
 * @param  {unknown[]} array
 * @param  {unknown[]} ...arrays
 */
export const arraysJoin = <A extends unknown[]>(
  array: unknown[],
  ...arrays: unknown[]
) => {
  return [...new Set(array.concat(...arrays))] as A;
};
