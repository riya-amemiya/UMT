import { startOf, type DateBoundaryUnit } from "./startOf";

/**
Returns true when `date` is strictly after `compare` at the given unit granularity.
Without a unit, compares exact timestamps (millisecond precision).
With a unit, both dates are truncated with `startOf` before comparison.
Week boundaries follow Sunday-start local time, matching `startOf`.

@param {Date} date - Date to test
@param {Date} compare - Date to compare against
@param {DateBoundaryUnit} [unit] - Granularity; omit for exact comparison
@returns {boolean} True when `date` is after `compare`
@example
isAfter(new Date(2025, 3, 16), new Date(2025, 3, 15)); // true
isAfter(new Date(2025, 3, 15, 23), new Date(2025, 3, 15, 1), "day"); // false
isAfter(new Date(2025, 4, 1), new Date(2025, 3, 30), "month"); // true
*/
export const isAfter = (
  date: Date,
  compare: Date,
  unit?: DateBoundaryUnit,
): boolean => {
  return unit === undefined
    ? date.getTime() > compare.getTime()
    : startOf(date, unit).getTime() > startOf(compare, unit).getTime();
};
