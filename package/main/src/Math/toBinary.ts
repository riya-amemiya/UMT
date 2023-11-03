/**
 * n進数に変換
 * @param {number} x
 * @param  {number} [radix=2] n進数
 * @returns string
 * @example toBinary(10); // "1010"
 */
export const toBinary = (x: number, radix = 2) => {
  return x.toString(radix);
};
