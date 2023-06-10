/**
 * 階乗
 * @param  {number} x
 * @returns number
 */
export const factorial = (x: number): number => {
  let result = 1;
  let copyX = x;
  if (copyX !== 0) {
    while (copyX > 1) {
      result *= copyX;
      copyX--;
    }
  }
  return result;
};
