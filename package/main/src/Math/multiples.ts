/**
 * 倍数
 * @param  {number} x
 * @param  {number} n
 */
export const multiples = (x: number, n: number) => {
  const result = [];
  for (let index = 1; index <= n; index++) {
    result.push(x * index);
  }
  return result;
};
