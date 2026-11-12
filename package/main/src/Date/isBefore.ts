import { startOf, type DateBoundaryUnit } from "./startOf";

/**
Returns true when `date` is strictly before `compare` at the given unit granularity.
Without a unit, compares exact timestamps (millisecond precision).
With a unit, both dates are truncated with `startOf` before comparison.
Week boundaries follow Sunday-start local time, matching `startOf`.

@param {Date} date - Date to test
@param {Date} compare - Date to compare against
@param {DateBoundaryUnit} [unit] - Granularity; omit for exact comparison
@returns {boolean} True when `date` is before `compare`
@example
isBefore(new Date(2025, 3, 15), new Date(2025, 3, 16)); // true
isBefore(new Date(2025, 3, 15, 1), new Date(2025, 3, 15, 23), "day"); // false
isBefore(new Date(2025, 3, 30), new Date(2025, 4, 1), "month"); // true
*/
export const isBefore = (
  date: Date,
  compare: Date,
  unit?: DateBoundaryUnit,
): boolean => {
  return unit === undefined
    ? date.getTime() < compare.getTime()
    : startOf(date, unit).getTime() < startOf(compare, unit).getTime();
};
