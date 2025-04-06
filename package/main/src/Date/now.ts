import type { HoursTypeInt } from "$/clock/hoursTypeInt";
import { OneHourMs } from "@/Consts/clock";

/**
 * Get the current time with a specified UTC offset, regardless of the local timezone
 * @param {hoursTypeInt} [timeDifference=9] Hours offset from UTC (default: 9 for Japan Standard Time)
 * @returns {Date} Current date and time adjusted for the specified UTC offset
 * @example
 * now(); // Returns current time in JST (UTC+9)
 * now(0); // Returns current time in UTC
 * now(1); // Returns current time in UTC+1
 */
export const now = (timeDifference: HoursTypeInt = 9): Date => {
  return new Date(Date.now() + timeDifference * OneHourMs);
};
