/**
 * ビット回転（左回転）を行う関数
 * @param x 回転させる32ビット整数
 * @param k 回転するビット数
 * @returns 左にkビット回転された結果
 * @example
 * const result = bitwise(0x12345678, 8);
 * console.log(result.toString(16)); // '34567812'
 */
export const bitwise = (x: number, k: number): number =>
  (x << k) | (x >>> (32 - k));
