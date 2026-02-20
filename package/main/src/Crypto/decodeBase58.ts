/**
 * Decodes a Base58 string to Uint8Array
 * @param {string} input - Base58 encoded string
 * @returns {Uint8Array} Decoded bytes
 * @example decodeBase58("9Ajdvzr"); // Uint8Array for "Hello"
 */
export const decodeBase58 = (input: string): Uint8Array => {
  const alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
  let bigNumber = 0n;

  for (const char of input) {
    const value = alphabet.indexOf(char);
    bigNumber = bigNumber * 58n + BigInt(value);
  }

  const bytes: number[] = [];
  while (bigNumber > 0) {
    bytes.unshift(Number(bigNumber % 256n));
    bigNumber /= 256n;
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
