/**
 * Clamps a number between a minimum and maximum value.
 *
 * @param value - The number to clamp.
 * @param min - The minimum bound.
 * @param max - The maximum bound.
 * @returns The clamped number.
 *
 * @example
 * ```typescript
 * clamp(5, 0, 10);   // 5
 * clamp(-3, 0, 10);  // 0
 * clamp(15, 0, 10);  // 10
 * ```
 */
export const clamp = (value: number, min: number, max: number): number => {
  return Math.min(Math.max(value, min), max);
};
