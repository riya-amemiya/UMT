/**
 * Removes HTML/XML tags from a string, keeping the text content.
 * Unlike `escapeHtml`, which escapes special characters, this deletes the
 * tags entirely.
 * @param {string} string_ - Input string
 * @returns {string} String with tags removed
 * @example
 * stripTags("<p>Hello <b>World</b></p>"); // "Hello World"
 */
export const stripTags = (string_: string): string =>
  string_.replaceAll(/<[^>]*>/g, "");
