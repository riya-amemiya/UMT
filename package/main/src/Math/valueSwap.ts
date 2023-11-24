/**
 * x < yになるように入れ替える
 * @param  {number} x
 * @param  {number} y
 * @return {[number, number]}
 * @example valueSwap(2, 1); // [1, 2]
 */
export const valueSwap = (x: number, y: number): [number, number] => {
  let temporary: number;
  if (y < x) {
    temporary = y;
    y = x;
    x = temporary;
  }
  return [x, y];
};
