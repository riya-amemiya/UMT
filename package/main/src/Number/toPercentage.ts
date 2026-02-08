/**
 * Calculates the percentage of a value relative to a total.
 *
 * Returns 0 when the total is 0 to avoid division by zero.
 *
 * @param value - The partial value
 * @param total - The total value
 * @param decimals - The number of decimal places (default 2)
 * @returns The percentage value
 *
 * @example
 * ```typescript
 * toPercentage(25, 100);      // 25
 * toPercentage(1, 3);         // 33.33
 * toPercentage(1, 3, 0);      // 33
 * toPercentage(0, 0);         // 0
 * toPercentage(50, 200, 1);   // 25.0 -> 25
 * ```
 */
export const toPercentage = (
  value: number,
  total: number,
  decimals = 2,
): number => {
  if (total === 0) {
    return 0;
  }

  const factor = 10 ** decimals;
  return Math.round((value / total) * 100 * factor) / factor;
};
