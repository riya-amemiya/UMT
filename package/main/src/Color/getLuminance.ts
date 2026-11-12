const LINEAR_THRESHOLD = 3928 / 100_000;

const toLinear = (channel: number): number => {
  const srgb = channel / 255;
  return srgb <= LINEAR_THRESHOLD
    ? srgb / 12.92
    : ((srgb + 0.055) / 1.055) ** 2.4;
};

/**
Calculate the WCAG 2.1 relative luminance of an RGBA color
@param rgba Object containing r, g, b values (0-255) and optional a (0-1); alpha does not affect the result
@returns {number} Relative luminance from 0 (black) to 1 (white)
@example getLuminance({ r: 255, g: 255, b: 255 }); // 1
@example getLuminance({ r: 255, g: 0, b: 0, a: 1 }); // 0.2126
*/
export const getLuminance = ({
  r,
  g,
  b,
}: {
  r: number;
  g: number;
  b: number;
  a?: number;
}): number =>
  0.2126 * toLinear(r) + 0.7152 * toLinear(g) + 0.0722 * toLinear(b);
