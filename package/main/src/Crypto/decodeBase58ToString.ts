import { decodeBase58 } from "./decodeBase58";

/**
 * Decodes a Base58 string to a UTF-8 string
 * @param {string} input - Base58 encoded string
 * @returns {string} Decoded string
 * @example decodeBase58ToString("9Ajdvzr"); // "Hello"
 */
export const decodeBase58ToString = (input: string): string => {
  return new TextDecoder().decode(decodeBase58(input));
};
