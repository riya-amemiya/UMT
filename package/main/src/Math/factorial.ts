/**
 * 階乗
 * @param  {number} x
 * @returns number
 * @example factorial(5); // 120
 */
export const factorial = (x: number): number => {
  const limit = Math.max(1, x);
  let result = 1;

  for (let index = 2; index <= limit; index++) {
    result *= index;
  }

  return result;
};
