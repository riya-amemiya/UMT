import type { DurationUnit } from "./durationUnit";
import { msByUnit } from "./msByUnit";

/**
 * Adds a duration to a date and returns a new Date.
 * Calendar-aware for "M" (months) and "y" (years), so end-of-month
 * is preserved (e.g. Jan 31 + 1 month = Feb 28/29).
 *
 * @param {Date} date - Base date
 * @param {number} amount - Amount to add (can be negative)
 * @param {DurationUnit} unit - Unit of the amount
 * @returns {Date} A new Date with the duration added
 * @example
 * addDuration(new Date("2025-01-31"), 1, "M"); // 2025-02-28
 * addDuration(new Date("2025-01-01"), 7, "d"); // 2025-01-08
 */
export const addDuration = (
  date: Date,
  amount: number,
  unit: DurationUnit,
): Date => {
  const ms = msByUnit[unit];
  if (ms !== undefined) {
    return new Date(date.getTime() + amount * ms);
  }

  const result = new Date(date);
  if (unit === "M") {
    const targetMonth = result.getMonth() + amount;
    const day = result.getDate();
    result.setDate(1);
    result.setMonth(targetMonth);
    const lastDayOfTargetMonth = new Date(
      result.getFullYear(),
      result.getMonth() + 1,
      0,
    ).getDate();
    result.setDate(Math.min(day, lastDayOfTargetMonth));
    return result;
  }

  const targetYear = result.getFullYear() + amount;
  const month = result.getMonth();
  const day = result.getDate();
  result.setDate(1);
  result.setFullYear(targetYear);
  result.setMonth(month);
  const lastDayOfTargetMonth = new Date(targetYear, month + 1, 0).getDate();
  result.setDate(Math.min(day, lastDayOfTargetMonth));
  return result;
};
