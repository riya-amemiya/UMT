import { escapeRegExp } from "@/Tool/escapeRegExp";

const formatTokenPattern =
  /\[([^[\]]+)]|YYYY|YY|MM|M|DD|D|d|HH|H|hh|h|mm|m|ss|s|SSS|A|a|ZZ|Z|([\s\S])/g;

const tokenPatterns: Record<string, string> = {
  YYYY: "[0-9]{4}",
  YY: "[0-9]{2}",
  MM: "[0-9]{2}",
  M: "[0-9]{1,2}",
  DD: "[0-9]{2}",
  D: "[0-9]{1,2}",
  d: "[0-6]",
  HH: "[0-9]{2}",
  H: "[0-9]{1,2}",
  hh: "[0-9]{2}",
  h: "[0-9]{1,2}",
  mm: "[0-9]{2}",
  m: "[0-9]{1,2}",
  ss: "[0-9]{2}",
  s: "[0-9]{1,2}",
  SSS: "[0-9]{3}",
  A: "AM|PM",
  a: "am|pm",
  ZZ: "[+-][0-9]{4}",
  Z: "[+-][0-9]{2}:[0-9]{2}",
};

interface ParsedFields {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  millisecond: number;
  twelveHour: boolean;
  pm?: boolean;
  dayOfWeek?: number;
  offset?: number;
  offsetInRange: boolean;
}

const applyToken = (
  fields: ParsedFields,
  token: string,
  value: string,
): ParsedFields => {
  switch (token) {
    case "YYYY": {
      return { ...fields, year: Number(value) };
    }
    case "YY": {
      return { ...fields, year: 2000 + Number(value) };
    }
    case "MM":
    case "M": {
      return { ...fields, month: Number(value) };
    }
    case "DD":
    case "D": {
      return { ...fields, day: Number(value) };
    }
    case "d": {
      return { ...fields, dayOfWeek: Number(value) };
    }
    case "HH":
    case "H": {
      return { ...fields, hour: Number(value), twelveHour: false };
    }
    case "hh":
    case "h": {
      return { ...fields, hour: Number(value), twelveHour: true };
    }
    case "mm":
    case "m": {
      return { ...fields, minute: Number(value) };
    }
    case "ss":
    case "s": {
      return { ...fields, second: Number(value) };
    }
    case "SSS": {
      return { ...fields, millisecond: Number(value) };
    }
    case "A":
    case "a": {
      return { ...fields, pm: value.toLowerCase() === "pm" };
    }
    default: {
      const digits = value.replace(":", "");
      const hours = Number(digits.slice(1, 3));
      const minutes = Number(digits.slice(3, 5));
      return {
        ...fields,
        offset: (digits.startsWith("-") ? -1 : 1) * (hours * 60 + minutes),
        offsetInRange: hours <= 23 && minutes <= 59,
      };
    }
  }
};

/**
Parses a string into a Date according to a format pattern.
Uses the same tokens as `format`: YYYY, YY, MM, M, DD, D, d, HH, H, hh, h,
mm, m, ss, s, SSS, A, a, Z, ZZ, and `[text]` for escaped literal text.
Any other character in the pattern must match literally.

- `YY` is read as 2000-2099.
- Fields missing from the pattern default to 1970-01-01 00:00:00.000.
- `hh` / `h` are converted with `A` / `a` when present (12 AM is 0, 12 PM is 12);
  without a meridiem token the hour is used as written.
- `d` (day of week, 0 = Sunday) must agree with the parsed calendar date.
- With `Z` / `ZZ`, the fields are read in that offset; otherwise in local time.
- When a token appears more than once, the last occurrence wins.

Returns an Invalid Date when the string does not match the pattern or a field
is out of range (month 1-12, day within the month, hour 0-23 or 1-12 for
12-hour tokens, minute and second 0-59, offset hours 0-23 and minutes 0-59).

@param {string} string - The date string to parse
@param {string} formatString - The format pattern string
@returns {Date} The parsed Date, or an Invalid Date when parsing fails
@example
parseDate("2025-04-04", "YYYY-MM-DD"); // 2025-04-04T00:00:00 local time
parseDate("04/04/2025 3:30 PM", "MM/DD/YYYY h:mm A"); // 2025-04-04T15:30:00 local time
parseDate("2025-04-04T15:30:00+09:00", "YYYY-MM-DDTHH:mm:ssZ"); // 2025-04-04T06:30:00Z
parseDate("2025-13-01", "YYYY-MM-DD"); // Invalid Date
*/
export const parseDate = (string: string, formatString: string): Date => {
  const tokens: string[] = [];
  const source = formatString.replaceAll(
    formatTokenPattern,
    (match: string, escaped?: string, literal?: string) => {
      const text = escaped ?? literal;
      if (text !== undefined) {
        return escapeRegExp(text);
      }
      tokens.push(match);
      return `(${tokenPatterns[match]})`;
    },
  );
  const matched = new RegExp(`^${source}$`).exec(string);
  if (matched === null) {
    return new Date(Number.NaN);
  }

  const fields: ParsedFields = {
    year: 1970,
    month: 1,
    day: 1,
    hour: 0,
    minute: 0,
    second: 0,
    millisecond: 0,
    twelveHour: false,
    offsetInRange: true,
  };
  for (const [index, token] of tokens.entries()) {
    Object.assign(fields, applyToken(fields, token, matched[index + 1]));
  }

  const calendar = new Date(0);
  calendar.setUTCFullYear(fields.year, fields.month, 0);
  const lastDayOfMonth = calendar.getUTCDate();
  calendar.setUTCFullYear(fields.year, fields.month - 1, fields.day);

  const isValid =
    fields.month >= 1 &&
    fields.month <= 12 &&
    fields.day >= 1 &&
    fields.day <= lastDayOfMonth &&
    (fields.twelveHour
      ? fields.hour >= 1 && fields.hour <= 12
      : fields.hour <= 23) &&
    fields.minute <= 59 &&
    fields.second <= 59 &&
    (fields.dayOfWeek === undefined ||
      fields.dayOfWeek === calendar.getUTCDay()) &&
    fields.offsetInRange;
  if (!isValid) {
    return new Date(Number.NaN);
  }

  const hour =
    fields.twelveHour && fields.pm !== undefined
      ? (fields.hour % 12) + (fields.pm ? 12 : 0)
      : fields.hour;
  const result = new Date(0);
  if (fields.offset === undefined) {
    result.setFullYear(fields.year, fields.month - 1, fields.day);
    result.setHours(hour, fields.minute, fields.second, fields.millisecond);
    return result;
  }
  result.setUTCFullYear(fields.year, fields.month - 1, fields.day);
  result.setUTCHours(
    hour,
    fields.minute - fields.offset,
    fields.second,
    fields.millisecond,
  );
  return result;
};
