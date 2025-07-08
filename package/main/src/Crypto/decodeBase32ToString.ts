import { decodeBase32 } from "./decodeBase32";

/**
 * Decodes a Base32 string to a UTF-8 string
 * @param {string} input - Base32 encoded string
 * @returns {string} Decoded string
 * @example decodeBase32ToString("JBSWY3DP"); // "Hello"
 */
export const decodeBase32ToString = (input: string): string => {
  return new TextDecoder().decode(decodeBase32(input));
};
