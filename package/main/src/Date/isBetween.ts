import type { DateInclusivity } from "./dateInclusivity";
import { startOf, type DateBoundaryUnit } from "./startOf";

/**
 * Returns true when `date` lies between `start` and `end`.
 * Without a unit, compares exact timestamps.
 * With a unit, all three dates are truncated with `startOf` first.
 * Week boundaries follow Sunday-start local time, matching `startOf`.
 * Ranges are not swapped: if `start` is after `end`, the result is false.
 *
 * @param {Date} date - Date to test
 * @param {Date} start - Range start
 * @param {Date} end - Range end
 * @param {DateBoundaryUnit} [unit] - Granularity; omit for exact comparison
 * @param {DateInclusivity} [inclusivity="()"] - Bound inclusivity
 * @returns {boolean} True when the date is between the bounds
 * @example
 * isBetween(new Date(2025, 3, 15), new Date(2025, 3, 10), new Date(2025, 3, 20));
 * isBetween(start, start, end, undefined, "[]"); // true
 */
export const isBetween = (
  date: Date,
  start: Date,
  end: Date,
  unit?: DateBoundaryUnit,
  inclusivity: DateInclusivity = "()",
): boolean => {
  const value =
    unit === undefined ? date.getTime() : startOf(date, unit).getTime();
  const from =
    unit === undefined ? start.getTime() : startOf(start, unit).getTime();
  const to = unit === undefined ? end.getTime() : startOf(end, unit).getTime();
  const includeStart = inclusivity.startsWith("[");
  const includeEnd = inclusivity.endsWith("]");
  const afterStart = includeStart ? value >= from : value > from;
  const beforeEnd = includeEnd ? value <= to : value < to;
  return afterStart && beforeEnd;
};
