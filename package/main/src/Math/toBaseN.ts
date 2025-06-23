/**
 * Converts a number to a string representation in the specified base
 * @param {number} value The number to convert
 * @param {number} [radix=2] The base to convert to (2-36)
 * @returns {string} String representation of the number in the specified base
 * @example toBaseN(10); // "1010" (binary)
 * @example toBaseN(15, 16); // "f" (hexadecimal)
 * @example toBaseN(255, 16); // "ff" (hexadecimal)
 */
export const toBaseN = (value: number, radix = 2): string => {
  return value.toString(radix);
};
