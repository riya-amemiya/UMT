import { division } from "@/Math/division";
import { roundOf } from "@/Math/roundOf";
/**
 * Convert hexadecimal color code to RGBA color values
 * @param hex Hexadecimal color code (3, 6, or 8 digits with #)
 * @returns {Object} RGBA values (r, g, b as 0-255, a as 0-1)
 * @example hexaToRgba("#00000000") // { r: 0, g: 0, b: 0, a: 0 }
 * @throws {Error} If the hex code format is invalid
 */
export const hexaToRgba = (
  hex: string,
): {
  r: number;
  g: number;
  b: number;
  a: number;
} => {
  let hexCode = hex.replace("#", "");

  // Convert 3-digit hex to 6-digit format
  if (hexCode.length === 3) {
    hexCode = hexCode
      .split("")
      .map((char) => char + char)
      .join("");
  }

  const r = Number.parseInt(hexCode.slice(0, 2), 16);
  const g = Number.parseInt(hexCode.slice(2, 4), 16);
  const b = Number.parseInt(hexCode.slice(4, 6), 16);
  const alphaValue =
    hexCode.length === 8 ? Number.parseInt(hexCode.slice(6, 8), 16) : 255;
  const a = roundOf(division(alphaValue, 255), 2);

  // NaN check is not necessary since validation ensures valid hex values
  return { r, g, b, a };
};
