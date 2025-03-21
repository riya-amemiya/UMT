import { division } from "@/Math/division";
import { roundOf } from "@/Math/roundOf";
import { subtract } from "@/Math/subtract";

/**
 * Convert RGBA color to CMYK color model
 * @param rgba Object containing r, g, b values (0-255) and optional a (0-1)
 * @returns {Object} CMYK values (c, m, y, k as percentages 0-100) and alpha channel
 * @example rgbaToCmyk({ r: 0, g: 0, b: 0, a: 1 }); // { c: 0, m: 0, y: 0, k: 100, a: 1 }
 */
export const rgbaToCmyk = ({
  r,
  g,
  b,
  a = 1,
}: {
  r: number;
  g: number;
  b: number;
  a?: number;
}): { c: number; m: number; y: number; k: number; a: number } => {
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

  const rPrime = division(r, 255);
  const gPrime = division(g, 255);
  const bPrime = division(b, 255);

  const k = subtract(1, Math.max(rPrime, gPrime, bPrime));
  const c = division(subtract(subtract(1, rPrime), k), subtract(1, k)) || 0;
  const m = division(subtract(subtract(1, gPrime), k), subtract(1, k)) || 0;
  const y = division(subtract(subtract(1, bPrime), k), subtract(1, k)) || 0;

  return {
    c: roundOf(c * 100, 2),
    m: roundOf(m * 100, 2),
    y: roundOf(y * 100, 2),
    k: roundOf(k * 100, 2),
    a,
  };
};
