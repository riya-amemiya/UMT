import { getLuminance } from "./getLuminance";

/**
Calculate the WCAG 2.1 contrast ratio between two RGBA colors
@param color1 Object containing r, g, b values (0-255) and optional a (0-1)
@param color2 Object containing r, g, b values (0-255) and optional a (0-1)
@returns {number} Contrast ratio from 1 (identical luminance) to 21 (black on white); the argument order does not matter
@example getContrastRatio({ r: 0, g: 0, b: 0 }, { r: 255, g: 255, b: 255 }); // 21
*/
export const getContrastRatio = (
  color1: { r: number; g: number; b: number; a?: number },
  color2: { r: number; g: number; b: number; a?: number },
): number => {
  const luminance1 = getLuminance(color1);
  const luminance2 = getLuminance(color2);
  return (
    (Math.max(luminance1, luminance2) + 0.05) /
    (Math.min(luminance1, luminance2) + 0.05)
  );
};
