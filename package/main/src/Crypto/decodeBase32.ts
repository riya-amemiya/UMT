import { BASE32_ALPHABET } from "./constants";

// O(1) lookup table for Base32 character-to-index mapping
const base32CharToIndex = new Map(
  [...BASE32_ALPHABET].map((c, index) => [c, index]),
);

/**
 * Decodes a Base32 string to Uint8Array
 * @param {string} input - Base32 encoded string
 * @returns {Uint8Array} Decoded bytes
 * @example decodeBase32("JBSWY3DP"); // Uint8Array for "Hello"
 */
export const decodeBase32 = (input: string): Uint8Array => {
  const cleanedInput = input.replaceAll("=", "");
  const result: number[] = [];
  let buffer = 0;
  let bufferLength = 0;

  for (const char of cleanedInput) {
    // Security: reject invalid characters to prevent silent data corruption
    const value = base32CharToIndex.get(char);
    if (value === undefined) {
      throw new Error(`Invalid Base32 character: "${char}"`);
    }

    buffer = (buffer << 5) | value;
    bufferLength += 5;

    if (bufferLength >= 8) {
      bufferLength -= 8;
      result.push((buffer >> bufferLength) & 0xff);
    }
  }

  return new Uint8Array(result);
};
