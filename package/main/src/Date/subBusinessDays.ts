import { addBusinessDays } from "./addBusinessDays";

/**
 * Subtracts business days from a date.
 * Equivalent to addBusinessDays(date, -amount, holidays).
 *
 * @param {Date} date - Base date
 * @param {number} amount - Business days to subtract
 * @param {Date[]} [holidays=[]] - Optional holiday dates
 * @returns {Date} A new Date after walking backward `amount` business days
 * @example
 * subBusinessDays(new Date(2025, 3, 21), 1); // 2025-04-18 (Friday)
 */
export const subBusinessDays = (
  date: Date,
  amount: number,
  holidays: Date[] = [],
): Date => addBusinessDays(date, -amount, holidays);
