/**
 * 共通しない要素をとりだす
 * @param  {unknown[]} array
 * @param  {unknown[]} ...arrays
 */
export const getArraysDiff = <A extends unknown[]>(
  array: unknown[],
  ...arrays: unknown[]
): A => {
  const result = array.concat(...arrays).filter((val, _index, arr) => {
    return arr.indexOf(val) === arr.lastIndexOf(val);
  });
  return result as A;
};
