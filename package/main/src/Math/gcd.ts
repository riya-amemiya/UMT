import { valueSwap } from "./valueSwap";

/**
 * 自然数の最大公約数
 * @param  {number} x
 * @param  {number} y
 * @param  {number} ...z
 * @returns number
 */
export const gcd = (x: number, y: number, ...z: number[]) => {
  let copyX = x;
  let copyY = y;
  const copyZ = z;
  if (copyX === 0 || copyY === 0) {
    return 0;
  }
  [copyX, copyY] = valueSwap(copyX, copyY);
  /* ユークリッドの互除法 */
  let r = copyY % copyX;
  while (r !== 0) {
    copyY = copyX;
    copyX = r;
    r = copyY % copyX;
  }
  if (copyZ.length > 0) {
    for (let i = 0; i < copyZ.length; i++) {
      copyX = gcd(copyX, copyZ[i]);
    }
  }
  return copyX;
};
