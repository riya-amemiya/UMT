/**
 * 階乗
 * @param  {number} x
 * @returns number
 * @example factorial(5); // 120
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
