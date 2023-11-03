/**
 * 倍数
 * @param  {number} x
 * @param  {number} n
 * @returns number[]
 * @example multiples(2, 5); // [2, 4, 6, 8, 10]
 */
export const multiples = (x: number, n: number) => {
  const result = [];
  for (let index = 1; index <= n; index++) {
    result.push(x * index);
  }
  return result;
};
