import type { Formatter } from "./formatter";

/**
 * Configuration options for formatString.
 *
 * @property formatters - Custom formatter functions to extend or override built-in formatters
 * @property locale - Default locale for built-in formatters (currently unused but reserved)
 *
 * @example
 * const options: FormatOptions = {
 *   formatters: {
 *     reverse: (value) => String(value).split('').reverse().join(''),
 *     shout: (value) => `${String(value).toUpperCase()}!`
 *   }
 * };
 * formatString("{text:reverse} {text:shout}", { text: "hello" }, options);
 * // → "olleh HELLO!"
 */
export interface FormatOptions {
  formatters?: Record<string, Formatter>;
  locale?: string;
}
