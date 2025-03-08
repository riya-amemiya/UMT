const hex = (x: number) => {
  const hexCode = x.toString(16);
  return hexCode.length === 1 ? `0${hexCode}` : hexCode;
};
/**
 * Convert RGBA color to hexadecimal color code
 * @param rgba Object containing r, g, b values (0-255) and optional a (0-1)
 * @returns {string} Hexadecimal color code including alpha channel
 * @example rgbaToHexA({ r: 0, g: 0, b: 0, a: 1 }); // "#000000ff"
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
  // Validate RGBA values
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
