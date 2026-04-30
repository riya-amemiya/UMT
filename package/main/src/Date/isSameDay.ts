/**
 * Returns true when two dates represent the same calendar day in local time.
 * @param {Date} left - First date
 * @param {Date} right - Second date
 * @returns {boolean} True if same year/month/day in local time
 * @example
 * isSameDay(new Date("2025-04-15T01:00"), new Date("2025-04-15T23:00")); // true
 * isSameDay(new Date("2025-04-15"), new Date("2025-04-16")); // false
 */
export const isSameDay = (left: Date, right: Date): boolean =>
  left.getFullYear() === right.getFullYear() &&
  left.getMonth() === right.getMonth() &&
  left.getDate() === right.getDate();
