/**
 * Encodes a string or Uint8Array to Base32 format
 * @param {string | Uint8Array} input - The input to encode
 * @returns {string} Base32 encoded string
 * @example encodeBase32("Hello"); // "JBSWY3DP"
 */
export const encodeBase32 = (input: string | Uint8Array): string => {
  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
  const bytes =
    typeof input === "string" ? new TextEncoder().encode(input) : input;

  let result = "";
  let buffer = 0;
  let bufferLength = 0;

  for (const byte of bytes) {
    buffer = (buffer << 8) | byte;
    bufferLength += 8;

    while (bufferLength >= 5) {
      bufferLength -= 5;
      result += alphabet[(buffer >> bufferLength) & 0x1f];
    }
  }

  if (bufferLength > 0) {
    result += alphabet[(buffer << (5 - bufferLength)) & 0x1f];
  }

  const paddingLength = (8 - (result.length % 8)) % 8;
  result += "=".repeat(paddingLength);

  return result;
};
