/**
 * Pads the start of a string with another string until the target length is reached
 *
 * @param {string} string_ - The original string to pad
 * @param {number} targetLength - The target length after padding
 * @param {string} padString - The string to use for padding
 * @returns {string} The padded string
 * @example
 * ```typescript
 * padStart("123", 5, "0");     // Returns: "00123"
 * padStart("abc", 8, "def");   // Returns: "defdeabc"
 * ```
 */
export const padStart = (
  string_: string,
  targetLength: number,
  padString: string,
): string => {
  // Return original string if padString is empty or string is already long enough
  if (padString === "" || string_.length >= targetLength) {
    return string_;
  }

  let padding = "";
  const paddingLength = targetLength - string_.length;

  // Build padding by repeating padString
  while (padding.length < paddingLength) {
    padding += padString;
  }

  // Trim padding to exact length needed and concatenate with original string
  return padding.slice(0, paddingLength) + string_;
};
