/**
 * 共通の要素をとりだす
 * @param  {any[]} array
 * @param  {any[]} ...arrays
 * @returns unknown[]
 * @example getArraysCommon([1, 2, 3], [2, 3, 4]); // [2, 3]
 */

export const getArraysCommon = <O, T extends unknown[] = unknown[]>(
  array: T,
  ...arrays: T[]
) => {
  const result = [array, ...arrays].reduce((previous, current) => {
    return previous.filter((item: unknown) => current.includes(item)) as T;
  });
  return result as unknown as O;
};
