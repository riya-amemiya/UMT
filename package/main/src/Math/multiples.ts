/**
 * Generate an array of multiples of a number
 * @param  {number} x Base number
 * @param  {number} n Number of multiples to generate
 * @returns {number[]} Array of multiples
 * @example multiples(2, 5); // [2, 4, 6, 8, 10]
 */
export const multiples = (x: number, n: number) => {
  const result: number[] = [];
  for (let index = 1; index <= n; index++) {
    result.push(x * index);
  }
  return result;
};
