/**
 * Removes the given prefix from the start of a string once, if present.
 * Unlike `trimStartCharacters`, which removes any of a set of characters
 * repeatedly, this removes the exact prefix a single time.
 * @param {string} string_ - Input string
 * @param {string} prefix - Prefix to remove
 * @returns {string} String without the leading prefix
 * @example
 * removePrefix("https://example.com", "https://"); // "example.com"
 * removePrefix("example.com", "https://"); // "example.com"
 */
export const removePrefix = (string_: string, prefix: string): string =>
  prefix !== "" && string_.startsWith(prefix)
    ? string_.slice(prefix.length)
    : string_;
