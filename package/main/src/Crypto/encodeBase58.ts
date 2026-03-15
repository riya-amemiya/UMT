import { BASE58_ALPHABET } from "./constants";

/**
 * Encodes a string or Uint8Array to Base58 format
 * @param {string | Uint8Array} input - The input to encode
 * @returns {string} Base58 encoded string
 * @example encodeBase58("Hello"); // "9Ajdvzr"
 */
export const encodeBase58 = (input: string | Uint8Array): string => {
  const alphabet = BASE58_ALPHABET;
  const bytes =
    typeof input === "string" ? new TextEncoder().encode(input) : input;

  // Use array + reverse + join instead of string prepend to avoid O(n²) string concatenation
  const chars: string[] = [];
  let bigNumber = 0n;

  for (const byte of bytes) {
    bigNumber = bigNumber * 256n + BigInt(byte);
  }

  while (bigNumber > 0) {
    const remainder = Number(bigNumber % 58n);
    chars.push(alphabet[remainder]);
    bigNumber /= 58n;
  }
  chars.reverse();
  const encoded = chars.join("");

  let leadingZeros = 0;
  for (const byte of bytes) {
    if (byte !== 0) {
      break;
    }
    leadingZeros++;
  }

  return "1".repeat(leadingZeros) + encoded;
};
