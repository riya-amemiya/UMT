/**
 * Removes specified characters from the end of a string
 *
 * @param {string} string_ - The input string to trim
 * @param {string} chars - Characters to remove from the end
 * @returns {string} A new string with specified characters removed from the end
 * @example
 * ```typescript
 * trimEndCharacters("hello!!!", "!"); // Returns: "hello"
 * trimEndCharacters("123---", "-");   // Returns: "123"
 * trimEndCharacters("abc123", "xyz"); // Returns: "abc123"
 * ```
 */
export const trimEndCharacters = (string_: string, chars: string): string => {
  let endIndex = string_.length - 1;
  while (endIndex >= 0 && chars.includes(string_[endIndex])) {
    endIndex--;
  }
  return string_.slice(0, endIndex + 1);
};
