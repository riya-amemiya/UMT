import {
  OneDayMs,
  OneHourMs,
  OneMinuteMs,
  OneMonthMs,
  OneSecondMs,
  OneWeekMs,
  OneYearMs,
} from "../Consts/clock";

const THRESHOLDS: { unit: Intl.RelativeTimeFormatUnit; ms: number }[] = [
  { unit: "year", ms: OneYearMs },
  { unit: "month", ms: OneMonthMs },
  { unit: "week", ms: OneWeekMs },
  { unit: "day", ms: OneDayMs },
  { unit: "hour", ms: OneHourMs },
  { unit: "minute", ms: OneMinuteMs },
  { unit: "second", ms: OneSecondMs },
];

/**
 * Formats a date relative to a base date using Intl.RelativeTimeFormat.
 * Picks the largest unit that fits the absolute delta.
 *
 * @param {Date} date - The target date
 * @param {Date} [baseDate=new Date()] - Reference date (defaults to now)
 * @param {string} [locale] - BCP 47 locale tag; uses system locale if omitted
 * @returns {string} Localized relative-time string
 * @example
 * formatRelative(new Date(Date.now() - 3600_000), new Date(), "en"); // "1 hour ago"
 * formatRelative(new Date(Date.now() + 86_400_000), new Date(), "ja"); // "1 日後"
 */
export const formatRelative = (
  date: Date,
  baseDate: Date = new Date(),
  locale?: string,
): string => {
  const deltaMs = date.getTime() - baseDate.getTime();
  const absDelta = Math.abs(deltaMs);
  const formatter = new Intl.RelativeTimeFormat(locale, { numeric: "auto" });

  for (const { unit, ms } of THRESHOLDS) {
    if (absDelta >= ms) {
      const value = Math.round(deltaMs / ms);
      return formatter.format(value, unit);
    }
  }
  return formatter.format(0, "second");
};
