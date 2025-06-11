import { applyFormatter } from "./formatString/applyFormatter";
import { defaultFormatters } from "./formatString/defaultFormatters";
import { detectMode } from "./formatString/detectMode";
import { getValue } from "./formatString/getValue";

import type { FormatData } from "$/string/formatData";
import type { FormatOptions } from "$/string/formatOptions";
import type { FormatValue } from "$/string/formatValue";

/**
 * Replaces placeholders in a template string with specified values.
 *
 * Supports two modes:
 * 1. **Indexed mode**: Use numbered placeholders like {0}, {1}, {2}...
 * 2. **Named mode**: Use named placeholders with an object like {name}, {age}...
 *
 * ## Advanced Features
 *
 * ### Nested Object Access
 * Access nested properties using dot notation: `{user.name}`, `{user.address.city}`
 *
 * ### Array Access
 * Access array elements with brackets: `{items[0]}`, `{users[1].name}`
 * Supports negative indices: `{items[-1]}` (last element), `{items[-2]}` (second to last)
 *
 * ### Default Values
 * Provide fallback values using pipe syntax: `{name|Unknown}`, `{age|N/A}`
 *
 * ### Formatters
 * Apply formatters to values: `{name:upper}`, `{price:currency}`, `{date:date}`
 *
 * Built-in formatters:
 * - `upper` - Convert to uppercase
 * - `lower` - Convert to lowercase
 * - `currency(locale?, currency?)` - Format as currency (e.g., `{price:currency(ja-JP,JPY)}`)
 * - `date(locale?, format?)` - Format dates (format: short|medium|long|full|iso|time)
 * - `time(locale?)` - Format time
 * - `number(locale?, minFraction?, maxFraction?)` - Format numbers
 * - `plural(singular, plural)` - Pluralization (e.g., `{count:plural(item,items)}`)
 * - `pad(length, char?)` - Pad string/number (e.g., `{id:pad(4,0)}`)
 *
 * ### Escape Sequences
 * Use double braces to escape: `{{name}}` renders as literal `{name}`
 *
 * @param template - Template string containing placeholders
 * @param data - Data object for named mode OR first value for indexed mode
 * @param options - Options object with custom formatters (named mode only)
 * @returns String with placeholders replaced with formatted values
 *
 * @example
 * // Indexed mode - basic replacement
 * formatString("Hello, {0}! It's {1} today.", "World", "sunny");
 * // → "Hello, World! It's sunny today."
 *
 * @example
 * // Named mode - object properties
 * formatString("Hello, {name}! You are {age} years old.", { name: "Alice", age: 25 });
 * // → "Hello, Alice! You are 25 years old."
 *
 * @example
 * // Nested object access
 * formatString("User: {user.name}, Email: {user.email}", {
 *   user: { name: "Charlie", email: "charlie@example.com" }
 * });
 * // → "User: Charlie, Email: charlie@example.com"
 *
 * @example
 * // Array access with negative indices
 * formatString("First: {items[0]}, Last: {items[-1]}", { items: ["A", "B", "C"] });
 * // → "First: A, Last: C"
 *
 * @example
 * // Default values
 * formatString("Name: {name|Unknown}, Age: {age|N/A}", { age: 25 });
 * // → "Name: Unknown, Age: 25"
 *
 * @example
 * // Built-in formatters
 * formatString("Price: {price:currency(ja-JP,JPY)}", { price: 1234 });
 * // → "Price: ￥1,234"
 *
 * @example
 * // Custom formatters
 * formatString("Reversed: {text:reverse}", { text: "hello" }, {
 *   formatters: {
 *     reverse: (value) => String(value).split("").reverse().join("")
 *   }
 * });
 * // → "Reversed: olleh"
 *
 * @example
 * // Escape sequences
 * formatString("Literal {{name}} and value {name}", { name: "Alice" });
 * // → "Literal {name} and value Alice"
 */
export function formatString(
  template: string,
  data?: FormatData,
  options?: FormatOptions,
): string;
export function formatString(
  template: string,
  ...values: FormatValue[]
): string;
export function formatString(
  template: string,
  dataOrFirstValue?: FormatData | FormatValue,
  optionsOrSecondValue?: FormatOptions | FormatValue,
  ...restValues: FormatValue[]
): string {
  const escapedTemplate = template
    .replaceAll("{{", "\u0000")
    .replaceAll("}}", "\u0001");

  const { data, options } = detectMode(
    dataOrFirstValue,
    optionsOrSecondValue,
    restValues,
  );

  const formatters = { ...defaultFormatters, ...options.formatters };

  const result = escapedTemplate.replaceAll(
    /{([^}]+)}/g,
    (match, content: string) => {
      const [pathAndFormatter, defaultValue] = content
        .split("|")
        .map((part) => part.trim());
      const [path, ...formatterParts] = pathAndFormatter.split(":");
      const formatterString = formatterParts.join(":");

      let value: unknown;
      if (Array.isArray(data)) {
        const index = Number(path);
        value = Number.isNaN(index) ? undefined : data[index];
      } else {
        value = getValue(data, path);
      }

      if (value === undefined || value === null) {
        if (defaultValue === undefined) {
          return match;
        }
        value = defaultValue;
      }

      if (formatterString) {
        return applyFormatter(value, formatterString, formatters);
      }

      return String(value);
    },
  );

  return result.replaceAll("\u0000", "{").replaceAll("\u0001", "}");
}
