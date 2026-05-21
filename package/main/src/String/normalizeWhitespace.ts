/**
 * Collapses consecutive whitespace characters into a single space and trims
 * the result. Unlike `deleteSpaces`, which removes all whitespace, this keeps
 * words separated by single spaces.
 * @param {string} string_ - Input string
 * @returns {string} String with normalized whitespace
 * @example
 * normalizeWhitespace("  hello   world \t\n foo "); // "hello world foo"
 */
export const normalizeWhitespace = (string_: string): string =>
  string_.replaceAll(/\s+/g, " ").trim();
