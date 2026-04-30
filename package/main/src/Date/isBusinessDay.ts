import { isSameDay } from "./isSameDay";
import { isWeekend } from "./isWeekend";

/**
 * Returns true when the given date is a weekday and not in the holidays list.
 * @param {Date} date - The date to check
 * @param {Date[]} [holidays=[]] - Optional list of holiday dates
 * @returns {boolean} True when weekday and not a listed holiday
 * @example
 * isBusinessDay(new Date("2025-04-21")); // true (Monday)
 * isBusinessDay(new Date("2025-04-21"), [new Date("2025-04-21")]); // false
 */
export const isBusinessDay = (date: Date, holidays: Date[] = []): boolean => {
  if (isWeekend(date)) {
    return false;
  }
  return !holidays.some((holiday) => isSameDay(holiday, date));
};
