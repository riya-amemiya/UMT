import type { UnixTimeUnit } from "./unixTimeUnit";

/**
 * Converts a Date to a Unix timestamp.
 * Default unit is seconds, floored to an integer.
 *
 * @param {Date} date - Date to convert
 * @param {UnixTimeUnit} [unit="s"] - Seconds or milliseconds
 * @returns {number} Unix timestamp in the requested unit
 * @example
 * toUnix(new Date(1_700_000_000_999)); // 1700000000
 * toUnix(new Date(1_700_000_000_123), "ms"); // 1700000000123
 */
export const toUnix = (date: Date, unit: UnixTimeUnit = "s"): number => {
  switch (unit) {
    case "s": {
      return Math.floor(date.getTime() / 1000);
    }
    case "ms": {
      return date.getTime();
    }
    default: {
      return ((_unit: never) => Math.floor(date.getTime() / 1000))(unit);
    }
  }
};
