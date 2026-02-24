import { clamp } from "@/Math/clamp";
import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";
import { roundOf } from "@/Math/roundOf";
import { subtract } from "@/Math/subtract";

/**
 * Convert CMYK color values to RGBA color space
 * @param {number} c Cyan percentage (0-100)
 * @param {number} m Magenta percentage (0-100)
 * @param {number} y Yellow percentage (0-100)
 * @param {number} k Key/Black percentage (0-100)
 * @param {number} a Alpha value (0-1)
 * @returns {Object} RGBA values (r, g, b as 0-255, a as 0-1)
 * @example cmykToRgba(100, 100, 0, 60.78) // { r: 0, g: 0, b: 100, a: 1 }
 */
export const cmykToRgba = (
  c: number,
  m: number,
  y: number,
  k: number,
  a = 1,
): { r: number; g: number; b: number; a: number } => {
  // Clamp CMYK values to 0-100 range
  const clampedC = clamp(c, 0, 100);
  const clampedM = clamp(m, 0, 100);
  const clampedY = clamp(y, 0, 100);
  const clampedK = clamp(k, 0, 100);

  // Convert CMYK values to 0-1 range
  const cPercentage = division(clampedC, 100);
  const mPercentage = division(clampedM, 100);
  const yPercentage = division(clampedY, 100);
  const kPercentage = division(clampedK, 100);

  // Calculate RGB values
  const r = multiplication(
    255,
    subtract(1, cPercentage),
    subtract(1, kPercentage),
  );
  const g = multiplication(
    255,
    subtract(1, mPercentage),
    subtract(1, kPercentage),
  );
  const b = multiplication(
    255,
    subtract(1, yPercentage),
    subtract(1, kPercentage),
  );

  // Round RGB values to 0-255 range
  const roundedR = roundOf(r);
  const roundedG = roundOf(g);
  const roundedB = roundOf(b);

  // Clamp alpha value to 0-1 range
  const clampedA = clamp(a, 0, 1);

  return {
    r: roundedR,
    g: roundedG,
    b: roundedB,
    a: clampedA,
  };
};
