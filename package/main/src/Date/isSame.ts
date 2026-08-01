import { startOf, type DateBoundaryUnit } from "./startOf";

/**
 * Returns true when two dates are the same at the given unit granularity.
 * Without a unit, compares exact timestamps (millisecond precision).
 * With a unit, both dates are truncated with `startOf` before comparison.
 * Week boundaries follow Sunday-start local time, matching `startOf`.
 *
 * @param {Date} left - First date
 * @param {Date} right - Second date
 * @param {DateBoundaryUnit} [unit] - Granularity; omit for exact equality
 * @returns {boolean} True when the dates match at the given granularity
 * @example
 * isSame(new Date(2025, 3, 15, 1), new Date(2025, 3, 15, 23), "day"); // true
 * isSame(new Date(2025, 3, 15), new Date(2025, 4, 15), "month"); // false
 * isSame(new Date(2025, 0, 1), new Date(2025, 0, 1)); // true
 */
export const isSame = (
  left: Date,
  right: Date,
  unit?: DateBoundaryUnit,
): boolean => {
  if (unit === undefined) {
    return left.getTime() === right.getTime();
  }
  return startOf(left, unit).getTime() === startOf(right, unit).getTime();
};
