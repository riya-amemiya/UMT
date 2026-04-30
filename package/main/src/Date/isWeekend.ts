/**
 * Returns true when the given date is Saturday or Sunday.
 * @param {Date} date - The date to check
 * @returns {boolean} True if Saturday or Sunday, false otherwise
 * @example
 * isWeekend(new Date("2025-04-19")); // true (Saturday)
 * isWeekend(new Date("2025-04-21")); // false (Monday)
 */
export const isWeekend = (date: Date): boolean => {
  const dayOfWeek = date.getDay();
  return dayOfWeek === 0 || dayOfWeek === 6;
};
