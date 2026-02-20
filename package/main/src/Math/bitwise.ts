/**
 * Performs bit rotation on a number
 * @param x 32-bit integer to rotate
 * @param k Number of bits to rotate
 * @param direction Direction of rotation ('left' or 'right')
 * @returns The result after rotating k bits in the specified direction
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
      return direction satisfies never;
    }
  }
};
