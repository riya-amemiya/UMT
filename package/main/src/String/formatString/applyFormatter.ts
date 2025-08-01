import type { Formatter } from "$/string/formatString/formatter";

/**
 * Applies a formatter function to a value with optional arguments.
 *
 * Parses formatter syntax like "upper", "currency(ja-JP,JPY)", "pad(4,0)" and applies
 * the corresponding formatter function with parsed arguments.
 *
 * @param value - The value to format
 * @param formatterString - Formatter name with optional arguments (e.g., "upper", "currency(ja-JP,JPY)")
 * @param formatters - Available formatter functions
 * @returns Formatted string, or original string value if formatter not found/invalid
 *
 * @example
 * // Simple formatter
 * applyFormatter("hello", "upper", { upper: (v) => String(v).toUpperCase() }) // → "HELLO"
 *
 * @example
 * // Formatter with arguments
 * applyFormatter(42, "pad(4,0)", { pad: (v, len, char) => String(v).padStart(+len, char) }) // → "0042"
 *
 * @example
 * // Invalid formatter returns original value
 * applyFormatter("test", "invalid!@#", {}) // → "test"
 */
export function applyFormatter(
  value: unknown,
  formatterString: string,
  formatters: Record<string, Formatter>,
): string {
  const match = formatterString.match(/^(\w+)(?:\(([^)]*)\))?$/);
  if (!match) {
    return String(value);
  }

  const [, formatterName, argumentsString] = match;
  const formatter = formatters[formatterName];

  if (!formatter) {
    return String(value);
  }

  const arguments_ = argumentsString ? parseArguments(argumentsString) : [];

  return formatter(value, ...arguments_);
}

/**
 * Parses comma-separated arguments while preserving quoted strings
 * @param argumentsString - String containing comma-separated arguments
 * @returns Array of parsed arguments
 */
function parseArguments(argumentsString: string): string[] {
  const arguments_: string[] = [];
  let current = "";
  let inQuotes = false;
  let quoteChar = "";

  for (const char of argumentsString) {
    if (!inQuotes && (char === '"' || char === "'")) {
      inQuotes = true;
      quoteChar = char;
      continue;
    }

    if (inQuotes && char === quoteChar) {
      inQuotes = false;
      quoteChar = "";
      continue;
    }

    if (!inQuotes && char === ",") {
      const trimmed = current.trim();
      arguments_.push(trimmed === "" ? " " : trimmed);
      current = "";
      continue;
    }

    current += char;
  }

  // Handle last argument
  const trimmed = current.trim();
  arguments_.push(trimmed === "" ? " " : trimmed);

  return arguments_;
}
