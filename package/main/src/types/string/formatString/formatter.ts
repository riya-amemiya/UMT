/**
 * Function type for formatString formatter functions.
 *
 * @param value - The value to be formatted
 * @param arguments_ - Additional string arguments passed to the formatter
 * @returns Formatted string representation of the value
 *
 * @example
 * const upperFormatter: Formatter = (value) => String(value).toUpperCase();
 * const padFormatter: Formatter = (value, length = "2", char = "0") =>
 *   String(value).padStart(Number(length), char);
 */
export type Formatter = (value: unknown, ...arguments_: string[]) => string;
