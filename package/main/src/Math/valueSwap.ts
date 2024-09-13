/**
 * x < yになるように入れ替える
 * @param  {number} x
 * @param  {number} y
 * @return {[number, number]}
 * @example valueSwap(2, 1); // [1, 2]
 */
export const valueSwap = (x: number, y: number): [number, number] => {
  return x < y ? [x, y] : [y, x];
};
