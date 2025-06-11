import type { Formatter } from "$/string/formatter";

/**
 * Built-in formatter functions for formatString.
 *
 * Each formatter takes a value and optional arguments, returning a formatted string.
 * Formatters are used with syntax like {value:formatterName} or {value:formatterName(arg1,arg2)}.
 *
 * Available formatters:
 * - upper: Convert to uppercase
 * - lower: Convert to lowercase
 * - currency: Format as currency with locale support
 * - date: Format dates with locale and format options
 * - time: Format time with locale support
 * - number: Format numbers with precision control
 * - plural: Choose singular/plural form based on count
 * - pad: Pad string with characters to specified length
 *
 * @example
 * // Usage in formatString
 * formatString("{name:upper}", { name: "alice" }) // → "ALICE"
 * formatString("{price:currency(ja-JP,JPY)}", { price: 1000 }) // → "￥1,000"
 * formatString("{count:plural(item,items)}", { count: 1 }) // → "item"
 */
export const defaultFormatters: Record<string, Formatter> = {
  upper: (value) => String(value).toUpperCase(),
  lower: (value) => String(value).toLowerCase(),

  currency: (value, locale = "en-US", currency = "USD") => {
    return new Intl.NumberFormat(locale, {
      style: "currency",
      currency,
    }).format(Number(value));
  },

  date: (value, locale = "en-US", format = "short") => {
    const date = value instanceof Date ? value : new Date(String(value));
    if (format === "iso") {
      return date.toISOString();
    }
    if (format === "time") {
      return date.toLocaleTimeString(locale);
    }
    return date.toLocaleDateString(locale);
  },

  time: (value, locale = "en-US") => {
    const date = value instanceof Date ? value : new Date(String(value));
    return date.toLocaleTimeString(locale);
  },

  number: (
    value,
    locale = "en-US",
    minimumFractionDigits = "0",
    maximumFractionDigits = "20",
  ) => {
    return new Intl.NumberFormat(locale, {
      minimumFractionDigits: Number(minimumFractionDigits),
      maximumFractionDigits: Number(maximumFractionDigits),
    }).format(Number(value));
  },

  plural: (value, singular, plural) => {
    return Number(value) === 1 ? singular : plural;
  },

  pad: (value, length = "2", char = "0") => {
    return String(value).padStart(Number(length), char);
  },
};
