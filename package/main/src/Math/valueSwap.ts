/**
 * x < yになるように入れ替える
 * @param  {number} x
 * @param  {number} y
 * @return {[number, number]}
 * @example valueSwap(2, 1); // [1, 2]
 */
export const valueSwap = (x: number, y: number): [number, number] => {
  let temporary: number;
  let copyX = x;
  let copyY = y;
  if (copyY < copyX) {
    temporary = copyY;
    copyY = copyX;
    copyX = temporary;
  }
  return [copyX, copyY];
};
