export function hexToRgba(hex: string): {
  r: number;
  g: number;
  b: number;
  a: number;
} {
  // hex形式かどうかのチェック
  if (/^#([0-9a-f]{3}|[0-9a-f]{6}|[0-9a-f]{8})$/i.test(hex) === false) {
    throw new Error("Invalid hex code");
  }

  const hexCode = hex.replace("#", "");
  const r = parseInt(hexCode.substring(0, 2), 16);
  const g = parseInt(hexCode.substring(2, 4), 16);
  const b = parseInt(hexCode.substring(4, 6), 16);
  const a = parseInt(hexCode.substring(6, 8), 16) / 255;
  return { r, g, b, a: Number.isNaN(a) ? 1 : a };
}
