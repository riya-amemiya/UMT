import { OneHourMs, OneMinuteMs, OneSecondMs } from "../Consts/clock";

import type { DurationUnit } from "./durationUnit";
import { msByUnit } from "./msByUnit";

/**
 * Returns the difference between two dates in the given unit, truncated toward zero.
 * Calendar-aware for "M" (months) and "y" (years).
 *
 * @param {Date} left - End date
 * @param {Date} right - Start date
 * @param {DurationUnit} unit - Unit of the result
 * @returns {number} The difference, truncated toward zero
 * @example
 * diff(new Date("2025-12-31"), new Date("2025-01-01"), "d"); // 364
 * diff(new Date("2026-01-01"), new Date("2025-01-01"), "y"); // 1
 */
export const diff = (left: Date, right: Date, unit: DurationUnit): number => {
  const ms = msByUnit[unit];
  if (ms !== undefined) {
    return Math.trunc((left.getTime() - right.getTime()) / ms);
  }

  const yearsDelta = left.getFullYear() - right.getFullYear();
  const monthsDelta = left.getMonth() - right.getMonth();
  const dayDelta = left.getDate() - right.getDate();
  const subDayDelta =
    left.getHours() * OneHourMs +
    left.getMinutes() * OneMinuteMs +
    left.getSeconds() * OneSecondMs +
    left.getMilliseconds() -
    (right.getHours() * OneHourMs +
      right.getMinutes() * OneMinuteMs +
      right.getSeconds() * OneSecondMs +
      right.getMilliseconds());

  let calendarMonths = yearsDelta * 12 + monthsDelta;
  if (
    calendarMonths > 0 &&
    (dayDelta < 0 || (dayDelta === 0 && subDayDelta < 0))
  ) {
    calendarMonths -= 1;
  } else if (
    calendarMonths < 0 &&
    (dayDelta > 0 || (dayDelta === 0 && subDayDelta > 0))
  ) {
    calendarMonths += 1;
  }

  if (unit === "M") {
    return calendarMonths;
  }
  return Math.trunc(calendarMonths / 12);
};
