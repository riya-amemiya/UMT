import { isBusinessDay } from "./isBusinessDay";

/**
 * Adds business days to a date, skipping weekends and holidays.
 * The starting date is not counted. A zero amount returns a clone.
 *
 * @param {Date} date - Base date
 * @param {number} amount - Business days to add (may be negative)
 * @param {Date[]} [holidays=[]] - Optional holiday dates
 * @returns {Date} A new Date after walking `amount` business days
 * @example
 * addBusinessDays(new Date(2025, 3, 18), 1); // 2025-04-21 (Monday)
 * addBusinessDays(new Date(2025, 3, 21), -1); // 2025-04-18 (Friday)
 */
export const addBusinessDays = (
  date: Date,
  amount: number,
  holidays: Date[] = [],
): Date => {
  const result = new Date(date);
  if (amount === 0) {
    return result;
  }
  const step = amount > 0 ? 1 : -1;
  let remaining = Math.abs(amount);
  while (remaining > 0) {
    result.setDate(result.getDate() + step);
    if (isBusinessDay(result, holidays)) {
      remaining -= 1;
    }
  }
  return result;
};
