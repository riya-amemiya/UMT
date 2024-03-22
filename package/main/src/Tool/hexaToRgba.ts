import { division } from "@/Math/division";
/**
 * hexaをrgbaに変換する
 * @param hex
 * @returns { r: number; g: number; b: number; a: number; }
 * @example hexToRgba("#00000000") // { r: 0, g: 0, b: 0, a: 0 }
 */
export const hexaToRgba = (
  hex: string,
): {
  r: number;
  g: number;
  b: number;
  a: number;
} => {
  // hex形式かどうかのチェック
  if (/^#([\da-f]{3}|[\da-f]{6}|[\da-f]{8})$/i.test(hex) === false) {
    throw new Error("Invalid hex code");
  }

  const hexCode = hex.replace("#", "");
  const r = Number.parseInt(hexCode.slice(0, 2), 16);
  const g = Number.parseInt(hexCode.slice(2, 4), 16);
  const b = Number.parseInt(hexCode.slice(4, 6), 16);
  const a = division(Number.parseInt(hexCode.slice(6, 8), 16), 255);
  return { r, g, b, a: Number.isNaN(a) ? 1 : a };
};
