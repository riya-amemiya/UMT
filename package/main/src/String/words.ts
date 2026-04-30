/**
 * Splits a string into words on case boundaries and non-alphanumeric separators.
 * Uses Unicode property escapes so CJK and other letters are preserved.
 *
 * @param {string} string_ - Input string
 * @param {RegExp} [pattern] - Custom matching pattern; defaults to a Unicode word matcher
 * @returns {string[]} Array of words
 * @example
 * words("helloWorld foo-bar"); // ["hello", "World", "foo", "bar"]
 * words("XMLHttpRequest"); // ["XML", "Http", "Request"]
 */
export const words = (string_: string, pattern?: RegExp): string[] => {
  if (pattern) {
    return string_.match(pattern) ?? [];
  }
  const withBoundaries = string_
    .replaceAll(/([\p{Ll}\p{N}])(\p{Lu})/gu, "$1 $2")
    .replaceAll(/(\p{Lu})(\p{Lu}\p{Ll})/gu, "$1 $2");
  return withBoundaries.split(/[^\p{L}\p{N}]+/u).filter(Boolean);
};
