import { getTimezoneOffsetString } from "@/Date/getTimezoneOffsetString";
import { padStart } from "@/String/padStart";

/**
 * Converts a date to a string according to the specified format pattern.
 * @param {Date} date - The date object to format
 * @param {string} formatString - The format pattern string (default: "YYYY-MM-DDTHH:mm:ssZ")
 * @return {string} - The formatted date string
 * @example
 * format(new Date('2025-04-04'), 'YYYY-MM-DD') // Returns "2025-04-04"
 * format(new Date('2025-04-04T15:30:00'), 'HH:mm') // Returns "15:30"
 * format(new Date('2025-04-04'), 'MM/DD/YYYY') // Returns "04/04/2025"
 *
 * Available format tokens:
 * - YYYY: Full year (e.g., 2025)
 * - YY: Short year (e.g., 25)
 * - MM: Month with leading zero (01-12)
 * - M: Month without leading zero (1-12)
 * - DD: Day with leading zero (01-31)
 * - D: Day without leading zero (1-31)
 * - d: Day of week (0-6)
 * - HH: Hours with leading zero (00-23)
 * - H: Hours without leading zero (0-23)
 * - hh: Hours (12-hour) with leading zero (01-12)
 * - h: Hours (12-hour) without leading zero (1-12)
 * - mm: Minutes with leading zero (00-59)
 * - m: Minutes without leading zero (0-59)
 * - ss: Seconds with leading zero (00-59)
 * - s: Seconds without leading zero (0-59)
 * - SSS: Milliseconds with leading zeros (000-999)
 * - A: AM/PM
 * - a: am/pm
 * - Z: Timezone offset (+09:00)
 * - ZZ: Timezone offset without colon (+0900)
 */
export const format = (
  date: Date,
  formatString = "YYYY-MM-DDTHH:mm:ssZ",
): string => {
  if (!(date instanceof Date)) {
    throw new TypeError("Invalid Date in format");
  }

  const hours = date.getHours();
  const yearString = String(date.getFullYear());
  const monthString = String(date.getMonth() + 1);
  const dateString = String(date.getDate());
  const hourString = String(hours);
  const minuteString = String(date.getMinutes());
  const secondString = String(date.getSeconds());
  const millisecondString = String(date.getMilliseconds());
  const dayString = String(date.getDay());
  const ampm = hours < 12 ? "AM" : "PM";
  const timezoneOffsetString = getTimezoneOffsetString(date);

  const matches: { [key: string]: string } = {
    YY: yearString.slice(-2),
    YYYY: padStart(yearString, 4, "0"),
    M: monthString,
    MM: padStart(monthString, 2, "0"),
    D: dateString,
    DD: padStart(dateString, 2, "0"),
    d: dayString,
    H: hourString,
    HH: padStart(hourString, 2, "0"),
    h: String(hours % 12 || 12),
    hh: padStart(String(hours % 12 || 12), 2, "0"),
    a: ampm.toLowerCase(),
    A: ampm,
    m: minuteString,
    mm: padStart(minuteString, 2, "0"),
    s: secondString,
    ss: padStart(secondString, 2, "0"),
    SSS: padStart(millisecondString, 3, "0"),
    Z: timezoneOffsetString,
    ZZ: timezoneOffsetString.replace(":", ""),
  };

  return formatString.replaceAll(
    /\[([^\]]+)]|(Y{1,4}|M{1,2}|D{1,2}|d{1,2}|H{1,2}|h{1,2}|a|A|m{1,2}|s{1,2}|Z{1,2}|SSS)/g,
    (_match, $1, $2) => $1 ?? matches[$2],
  );
};
