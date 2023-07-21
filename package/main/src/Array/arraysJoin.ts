/**
 * 重複をしないで結合
 * @param  {unknown[]} array
 * @param  {unknown[]} ...arrays
 */
export const arraysJoin = <
  A extends unknown[],
  B extends unknown[],
  C extends A & B,
>(
  array: A,
  ...arrays: B[]
) => {
  return [...new Set(array.concat(...arrays))] as C;
};
