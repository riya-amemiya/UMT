/**
 * 重複をしないで結合
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */
export const arraysJoin = <
  // rome-ignore lint/suspicious/noExplicitAny: <explanation>
  A extends any[],
  // rome-ignore lint/suspicious/noExplicitAny: <explanation>
  B extends any[],
  C extends A & B,
>(
  array: A,
  ...arrays: B[]
) => {
  return [...new Set(array.concat(...arrays))] as C;
};
