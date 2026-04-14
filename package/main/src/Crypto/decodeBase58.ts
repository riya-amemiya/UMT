import { BASE58_ALPHABET } from "./constants";

// O(1) lookup table for Base58 character-to-index mapping
const base58CharToIndex = new Map(
  [...BASE58_ALPHABET].map((c, index) => [c, index]),
);

/**
 * Decodes a Base58 string to Uint8Array
 * @param {string} input - Base58 encoded string
 * @returns {Uint8Array} Decoded bytes
 * @example decodeBase58("9Ajdvzr"); // Uint8Array for "Hello"
 */
export const decodeBase58 = (input: string): Uint8Array => {
  let bigNumber = 0n;

  for (const char of input) {
    bigNumber = bigNumber * 58n + BigInt(base58CharToIndex.get(char) ?? 0);
  }

  // Use push + reverse instead of unshift to avoid O(n²) array shifting
  const bytes: number[] = [];
  while (bigNumber > 0) {
    bytes.push(Number(bigNumber % 256n));
    bigNumber /= 256n;
  }
  bytes.reverse();

  let leadingOnes = 0;
  for (const char of input) {
    if (char !== "1") {
      break;
    }
    leadingOnes++;
  }

  return new Uint8Array([
    ...(Array.from({ length: leadingOnes }).fill(0) as number[]),
    ...bytes,
  ]);
};
