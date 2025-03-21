/**
 * Greatest Common Divisor (GCD)
 * @param  {number} x First number
 * @param  {number} y Second number
 * @param  {number[]} z Additional numbers (optional)
 * @returns number The GCD of all input numbers
 * @example gcd(12, 18); // 6
 */
export const gcd = (x: number, y: number, ...z: number[]) => {
  let copyX = Math.abs(x);
  let copyY = Math.abs(y);
  const copyZ = z.map((element) => Math.abs(element));
  if (copyX === 0 || copyY === 0) {
    return 0;
  }
  [copyX, copyY] = [Math.max(copyX, copyY), Math.min(copyX, copyY)];

  /* Euclidean algorithm */
  let r = copyY % copyX;
  while (r !== 0) {
    copyY = copyX;
    copyX = r;
    r = copyY % copyX;
  }
  if (copyZ.length > 0) {
    for (const element of copyZ) {
      copyX = gcd(copyX, element);
    }
  }
  return copyX;
};
