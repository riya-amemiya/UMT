/**
 * Ensures a string ends with the given suffix, appending it only when it is
 * not already present.
 * @param {string} string_ - Input string
 * @param {string} suffix - Suffix to ensure
 * @returns {string} String guaranteed to end with the suffix
 * @example
 * ensureSuffix("file", ".txt"); // "file.txt"
 * ensureSuffix("file.txt", ".txt"); // "file.txt"
 */
export const ensureSuffix = (string_: string, suffix: string): string =>
  string_.endsWith(suffix) ? string_ : string_ + suffix;
