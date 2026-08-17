import { OneDayMs } from "@/Consts/clock";

import { startOf } from "./startOf";

/**
 * Returns the Sunday-start week number of the date's local year.
 * Week 1 is the Sunday-start week that contains January 1.
 *
 * @param {Date} date - Date to inspect
 * @returns {number} Week number starting at 1
 * @example
 * weekOfYear(new Date(2025, 0, 1)); // 1
 * weekOfYear(new Date(2025, 0, 5)); // 2
 */
export const weekOfYear = (date: Date): number => {
  const weekStart = startOf(date, "week");
  const yearStartWeek = startOf(new Date(date.getFullYear(), 0, 1), "week");
  const days = Math.round(
    (weekStart.getTime() - yearStartWeek.getTime()) / OneDayMs,
  );
  return Math.floor(days / 7) + 1;
};
