/**
 * Swaps the case of each letter in a string, turning uppercase letters into
 * lowercase and vice versa. Characters without case are left unchanged.
 * @param {string} string_ - Input string
 * @returns {string} String with swapped case
 * @example
 * swapCase("Hello World"); // "hELLO wORLD"
 */
export const swapCase = (string_: string): string =>
  string_.replaceAll(/\p{L}/gu, (char) =>
    char === char.toLowerCase() ? char.toUpperCase() : char.toLowerCase(),
  );
