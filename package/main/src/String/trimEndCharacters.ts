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
  // Use Set for O(1) character lookups instead of O(m) string.includes()
  const charSet = new Set(chars);
  let endIndex = string_.length - 1;
  while (endIndex >= 0 && charSet.has(string_[endIndex])) {
    endIndex--;
  }
  return string_.slice(0, endIndex + 1);
};
