import { addition } from "@/Math/addition";
import { division } from "@/Math/division";
import { max } from "@/Math/max";
import { min } from "@/Math/min";
import { multiplication } from "@/Math/multiplication";
import { roundOf } from "@/Math/roundOf";
import { subtract } from "@/Math/subtract";

/**
 * Convert HSLA color values to RGBA color space
 * @param h Hue angle in degrees (0-360)
 * @param s Saturation percentage (0-100)
 * @param l Lightness percentage (0-100)
 * @param a Alpha value (0-1)
 * @returns {Object} RGBA values (r, g, b as 0-255, a as 0-1)
 * @example hslaToRgba(120, 50, 50, 1) // { r: 64, g: 191, b: 64, a: 1 }
 * @throws {Error} If any input values are out of their valid ranges
 */
export const hslaToRgba = (
  h: number,
  s: number,
  l: number,
  a = 1,
): { r: number; g: number; b: number; a: number } => {
  // Validate input ranges
  if (h < 0 || h > 360) {
    throw new Error("Hue must be between 0 and 360 degrees");
  }
  if (s < 0 || s > 100) {
    throw new Error("Saturation must be between 0 and 100 percent");
  }
  if (l < 0 || l > 100) {
    throw new Error("Lightness must be between 0 and 100 percent");
  }
  if (a < 0 || a > 1) {
    throw new Error("Alpha must be between 0 and 1");
  }

  const hue = division(division(h, 360, false)[1], 360);
  const saturation = division(max(0, min(s, 100)), 100);
  const lightness = division(max(0, min(l, 100)), 100);

  let r = 0;
  let g = 0;
  let b = 0;

  if (saturation === 0) {
    r = g = b = lightness;
  } else {
    const hueToRgb = (p: number, q: number, t: number) => {
      let tAdjusted = t;
      if (t < 0) {
        tAdjusted = addition(t, 1);
      }
      if (t > 1) {
        tAdjusted = subtract(t, 1);
      }
      if (tAdjusted < division(1, 6)) {
        return addition(
          p,
          multiplication(subtract(q, p), multiplication(6, tAdjusted)),
        );
      }
      if (tAdjusted < division(1, 2)) {
        return q;
      }
      if (tAdjusted < division(2, 3)) {
        return addition(
          p,
          multiplication(
            subtract(q, p),
            multiplication(subtract(division(2, 3, true), tAdjusted), 6),
          ),
        );
      }
      return p;
    };

    const q =
      lightness < 0.5
        ? multiplication(lightness, addition(1, saturation))
        : subtract(
            addition(lightness, saturation),
            multiplication(lightness, saturation),
          );
    const p = subtract(multiplication(2, lightness), q);

    r = hueToRgb(p, q, addition(hue, division(1, 3)));
    g = hueToRgb(p, q, hue);
    b = hueToRgb(p, q, subtract(hue, division(1, 3)));
  }

  const roundedR = roundOf(multiplication(r, 255), 2);
  const roundedG = roundOf(multiplication(g, 255), 2);
  const roundedB = roundOf(multiplication(b, 255), 2);
  const clampedA = max(0, min(1, a));

  return {
    r: roundedR,
    g: roundedG,
    b: roundedB,
    a: clampedA,
  };
};
