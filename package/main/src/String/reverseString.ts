/**
 * Reverses a string
 * @param {string} char - String to reverse
 * @returns Reversed string
 * @example reverseString("Hello"); // "olleH"
 */
export const reverseString = (char: string): string => {
  return char.split("").reverse().join("");
};
