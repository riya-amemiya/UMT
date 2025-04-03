/**
 * Removes specified characters from the start of a string
 *
 * @param {string} string_ - The input string to trim
 * @param {string} chars - Characters to remove from the start
 * @returns {string} A new string with specified characters removed from the start
 * @example
 * ```typescript
 * trimStartCharacters("!!!hello", "!"); // Returns: "hello"
 * trimStartCharacters("---123", "-");   // Returns: "123"
 * trimStartCharacters("abc123", "xyz"); // Returns: "abc123"
 * ```
 */
export const trimStartCharacters = (string_: string, chars: string): string => {
  let startIndex = 0;
  while (startIndex < string_.length && chars.includes(string_[startIndex])) {
    startIndex++;
  }
  return string_.slice(startIndex);
};
