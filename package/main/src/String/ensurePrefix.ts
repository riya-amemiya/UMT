/**
 * Ensures a string starts with the given prefix, prepending it only when it
 * is not already present.
 * @param {string} string_ - Input string
 * @param {string} prefix - Prefix to ensure
 * @returns {string} String guaranteed to start with the prefix
 * @example
 * ensurePrefix("example.com", "https://"); // "https://example.com"
 * ensurePrefix("https://example.com", "https://"); // "https://example.com"
 */
export const ensurePrefix = (string_: string, prefix: string): string =>
  string_.startsWith(prefix) ? string_ : prefix + string_;
