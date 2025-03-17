/**
 * Removes all whitespace characters from a string
 * @param {string} string_ - Input string
 * @returns {string} String with all whitespace characters removed
 * @example
 * ```typescript
 * deleteSpaces("Hello World");  // Returns: "HelloWorld"
 * deleteSpaces("  tab\t space ");  // Returns: "tabspace"
 * ```
 */
export const deleteSpaces = (string_: string): string => {
  return string_.replaceAll(/\s/g, "");
};
