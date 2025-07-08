/**
 * Decodes a Base58 string to Uint8Array
 * @param {string} input - Base58 encoded string
 * @returns {Uint8Array} Decoded bytes
 * @example decodeBase58("9Ajdvzr"); // Uint8Array for "Hello"
 */
export const decodeBase58 = (input: string): Uint8Array => {
  const alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
  let bigNumber = BigInt(0);

  for (const char of input) {
    const value = alphabet.indexOf(char);
    if (value === -1) {
      throw new Error(`Invalid base58 character: ${char}`);
    }
    bigNumber = bigNumber * BigInt(58) + BigInt(value);
  }

  const bytes: number[] = [];
  while (bigNumber > 0) {
    bytes.unshift(Number(bigNumber % BigInt(256)));
    bigNumber /= BigInt(256);
  }

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
