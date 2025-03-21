/**
 * Checks if the string contains no letters (contains only emojis, numbers, or special characters)
 * @param {string} text - The string to check
 * @returns {boolean} True if the string has no letters, false otherwise
 * @example
 * ```typescript
 * hasNoLetters("123");     // Returns: true
 * hasNoLetters("🌟123#"); // Returns: true
 * hasNoLetters("abc123"); // Returns: false
 * ```
 */
export const hasNoLetters = (text: string): boolean => {
  // Uses Unicode pattern:
  // \p{L} matches any Unicode letter
  // [^\p{L}]* matches zero or more non-letter characters
  const nonTextPattern = /^[^\p{L}]*$/u;
  return nonTextPattern.test(text);
};
