/**
 * 共通の要素をとりだす
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 */

export const getArraysCommon = <A extends unknown[]>(
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  array: any[],
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  ...arrays: any[]
) => {
  const result: unknown[] = [array, ...arrays].reduce((prev, current) => {
    return prev.filter((item: unknown) => current.includes(item));
  });
  return result as A;
};
