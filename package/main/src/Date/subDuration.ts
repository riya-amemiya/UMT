import { addDuration } from "./addDuration";
import type { DurationUnit } from "./durationUnit";

/**
 * Subtracts a duration from a date and returns a new Date.
 * Equivalent to addDuration(date, -amount, unit).
 *
 * @param {Date} date - Base date
 * @param {number} amount - Amount to subtract
 * @param {DurationUnit} unit - Unit of the amount
 * @returns {Date} A new Date with the duration subtracted
 * @example
 * subDuration(new Date("2025-03-31"), 1, "M"); // 2025-02-28
 */
export const subDuration = (
  date: Date,
  amount: number,
  unit: DurationUnit,
): Date => addDuration(date, -amount, unit);
