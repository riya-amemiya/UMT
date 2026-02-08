/**
 * Options for formatting a number.
 */
export interface FormatNumberOptions {
  /** The locale to use for formatting (e.g., "en-US", "ja-JP") */
  locale?: string;
  /** Minimum number of fraction digits */
  minimumFractionDigits?: number;
  /** Maximum number of fraction digits */
  maximumFractionDigits?: number;
  /** The formatting style */
  style?: "decimal" | "currency" | "percent";
  /** The currency code (required when style is "currency") */
  currency?: string;
}

/**
 * Formats a number using Intl.NumberFormat.
 *
 * @param value - The number to format
 * @param options - Formatting options
 * @returns The formatted number string
 *
 * @example
 * ```typescript
 * formatNumber(1234567.89);
 * // "1,234,567.89" (in en-US locale)
 *
 * formatNumber(1234567.89, { locale: "de-DE" });
 * // "1.234.567,89"
 *
 * formatNumber(0.75, { style: "percent" });
 * // "75%"
 *
 * formatNumber(1234.5, {
 *   style: "currency",
 *   currency: "USD",
 *   locale: "en-US",
 * });
 * // "$1,234.50"
 * ```
 */
export const formatNumber = (
  value: number,
  options: FormatNumberOptions = {},
): string => {
  const {
    locale,
    minimumFractionDigits,
    maximumFractionDigits,
    style,
    currency,
  } = options;

  const formatOptions: Intl.NumberFormatOptions = {};

  if (style !== undefined) {
    formatOptions.style = style;
  }
  if (currency !== undefined) {
    formatOptions.currency = currency;
  }
  if (minimumFractionDigits !== undefined) {
    formatOptions.minimumFractionDigits = minimumFractionDigits;
  }
  if (maximumFractionDigits !== undefined) {
    formatOptions.maximumFractionDigits = maximumFractionDigits;
  }

  return new Intl.NumberFormat(locale, formatOptions).format(value);
};
