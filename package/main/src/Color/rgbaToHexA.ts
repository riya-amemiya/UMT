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
  return `#${hex(r)}${hex(g)}${hex(b)}${hex(Math.round(a * 255))}`;
};
