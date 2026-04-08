import { BASE32_ALPHABET } from "./constants";

/**
 * Encodes a string or Uint8Array to Base32 format
 * @param {string | Uint8Array} input - The input to encode
 * @returns {string} Base32 encoded string
 * @example encodeBase32("Hello"); // "JBSWY3DP"
 */
export const encodeBase32 = (input: string | Uint8Array): string => {
  const alphabet = BASE32_ALPHABET;
  const bytes =
    typeof input === "string" ? new TextEncoder().encode(input) : input;

  // Performance: collect characters in an array and join at the end instead of
  // repeated string concatenation, which copies the entire string on every +=
  // and is O(n²) for large inputs. Array push + join is O(n).
  const chars: string[] = [];
  let buffer = 0;
  let bufferLength = 0;

  for (const byte of bytes) {
    buffer = (buffer << 8) | byte;
    bufferLength += 8;

    while (bufferLength >= 5) {
      bufferLength -= 5;
      chars.push(alphabet[(buffer >> bufferLength) & 0x1f]);
    }
  }

  if (bufferLength > 0) {
    chars.push(alphabet[(buffer << (5 - bufferLength)) & 0x1f]);
  }

  const paddingLength = (8 - (chars.length % 8)) % 8;
  if (paddingLength > 0) {
    chars.push("=".repeat(paddingLength));
  }

  return chars.join("");
};
