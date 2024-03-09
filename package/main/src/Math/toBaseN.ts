/**
 * n進数に変換
 * @param {number} x
 * @param  {number} [radix=2] n進数
 * @returns string
 * @example toBaseN(10); // "1010"
 */
export const toBaseN = (x: number, radix = 2) => {
  return x.toString(radix);
};
