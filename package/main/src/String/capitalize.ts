/**
 * Capitalizes the first grapheme of a string. Surrogate-pair safe.
 * Does not lowercase the rest of the string.
 *
 * @param {string} string_ - Input string
 * @returns {string} String with the first grapheme uppercased
 * @example
 * capitalize("hello"); // "Hello"
 * capitalize("éclair"); // "Éclair"
 * capitalize(""); // ""
 */
export const capitalize = (string_: string): string => {
  for (const first of string_) {
    return first.toUpperCase() + string_.slice(first.length);
  }
  return string_;
};
