/**
 * Calculate factorial of a number
 * @param  {number} x Number to calculate factorial for
 * @returns number The factorial of x
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
