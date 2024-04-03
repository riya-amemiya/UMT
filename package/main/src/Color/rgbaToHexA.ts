const hex = (x: number) => {
  const hexCode = x.toString(16);
  return hexCode.length === 1 ? `0${hexCode}` : hexCode;
};
/**
 * rgbaを16進数に変換
 * @param rgba
 * @returns {string} 16進数の色コード
 * @example rgbaToHexA({ r: 0, g: 0, b: 0, a: 1 }); // "#000000"
 */
export const rgbaToHexA = ({
  r,
  g,
  b,
  a = 1,
}: {
  r: number;
  g: number;
  b: number;
  a?: number;
}): string => {
  // rgba形式かどうかのチェック
  if (
    r < 0 ||
    r > 255 ||
    g < 0 ||
    g > 255 ||
    b < 0 ||
    b > 255 ||
    a < 0 ||
    a > 1
  ) {
    throw new Error("Invalid rgba value");
  }

  return `#${hex(r)}${hex(g)}${hex(b)}${hex(Math.round(a * 255))}`;
};
