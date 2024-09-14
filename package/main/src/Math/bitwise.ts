/**
 * ビット回転を行う関数
 * @param x 回転させる32ビット整数
 * @param k 回転するビット数
 * @param direction 回転方向 ('left' または 'right')
 * @returns 指定された方向にkビット回転された結果
 * @example
 * const resultLeft = bitwise(0x12345678, 8);
 * console.log(resultLeft.toString(16)); // '34567812'
 * const resultRight = bitwise(0x12345678, 8, 'right');
 * console.log(resultRight.toString(16)); // '78123456'
 */
export const bitwise = (
  x: number,
  k: number,
  direction: "left" | "right" = "left",
): number => {
  const rotation = ((k % 32) + 32) % 32;
  switch (direction) {
    case "left": {
      return (x << rotation) | (x >>> (32 - rotation));
    }
    case "right": {
      return (x >>> rotation) | (x << (32 - rotation));
    }
    default: {
      throw new Error(`Invalid direction ${direction}`);
    }
  }
};
