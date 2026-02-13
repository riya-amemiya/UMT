/**
 * Checks if a number is within a given range.
 * If only two arguments are provided, the range is [0, start).
 * If three arguments are provided, the range is [min(start, end), max(start, end)).
 *
 * @param value - The number to check.
 * @param start - The start of the range, or the end if end is not provided.
 * @param end - The end of the range (optional).
 * @returns True if the value is within the range, false otherwise.
 *
 * @example
 * ```typescript
 * inRange(3, 5);      // true  (range: [0, 5))
 * inRange(5, 5);      // false (range: [0, 5))
 * inRange(3, 2, 5);   // true  (range: [2, 5))
 * inRange(3, 5, 2);   // true  (range: [2, 5))
 * ```
 */
export const inRange = (
  value: number,
  start: number,
  end?: number,
): boolean => {
  const lower = end === undefined ? Math.min(start, 0) : Math.min(start, end);
  const upper = end === undefined ? Math.max(start, 0) : Math.max(start, end);
  return value >= lower && value < upper;
};
