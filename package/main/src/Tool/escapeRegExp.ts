/**
 * Escapes characters in a string for use in a regular expression.
 * @param {string} string - The string to escape.
 * @returns {string} The escaped string.
 * @example
 * escapeRegExp("a.b"); // "a\\.b"
 */
export const escapeRegExp = (string: string): string => {
  return string.replaceAll(/[.*+?^${}()|[\]\\]/g, String.raw`\$&`);
};
