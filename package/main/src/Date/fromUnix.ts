import type { UnixTimeUnit } from "./unixTimeUnit";

/**
 * Creates a Date from a Unix timestamp.
 * Default unit is seconds.
 *
 * @param {number} value - Timestamp value
 * @param {UnixTimeUnit} [unit="s"] - Seconds or milliseconds
 * @returns {Date} Date for the given epoch offset
 * @example
 * fromUnix(0).getTime(); // 0
 * fromUnix(1_700_000_000_123, "ms").getTime(); // 1700000000123
 */
export const fromUnix = (value: number, unit: UnixTimeUnit = "s"): Date => {
  switch (unit) {
    case "s": {
      return new Date(value * 1000);
    }
    case "ms": {
      return new Date(value);
    }
    default: {
      const _exhaustive: never = unit;
      return _exhaustive;
    }
  }
};
