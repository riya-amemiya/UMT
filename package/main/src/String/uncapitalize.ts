/**
 * Lowercases the first grapheme of a string. Surrogate-pair safe.
 *
 * @param {string} string_ - Input string
 * @returns {string} String with the first grapheme lowercased
 * @example
 * uncapitalize("Hello"); // "hello"
 * uncapitalize("ÉCLAIR"); // "éCLAIR"
 */
export const uncapitalize = (string_: string): string => {
  for (const first of string_) {
    return first.toLowerCase() + string_.slice(first.length);
  }
  return string_;
};
