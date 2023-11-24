/**
 * 階乗
 * @param  {number} x
 * @returns number
 * @example factorial(5); // 120
 */
export const factorial = (x: number): number => {
  let result = 1;

  if (x !== 0) {
    while (x > 1) {
      result *= x;
      x--;
    }
  }
  return result;
};
