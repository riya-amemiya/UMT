/**
 * x < yになるように入れ替える
 * @param  {number} x
 * @param  {number} y
 * @return {[number, number]}
 */
export const valueSwap = (x: number, y: number): [number, number] => {
  let tmp: number;
  let copyX = x;
  let copyY = y;
  if (copyY < copyX) {
    tmp = copyY;
    copyY = copyX;
    copyX = tmp;
  }
  return [copyX, copyY];
};
