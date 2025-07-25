/**
 * Encodes a string or Uint8Array to Base58 format
 * @param {string | Uint8Array} input - The input to encode
 * @returns {string} Base58 encoded string
 * @example encodeBase58("Hello"); // "9Ajdvzr"
 */
export const encodeBase58 = (input: string | Uint8Array): string => {
  const alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
  const bytes =
    typeof input === "string" ? new TextEncoder().encode(input) : input;

  let encoded = "";
  let bigNumber = BigInt(0);

  for (const byte of bytes) {
    bigNumber = bigNumber * BigInt(256) + BigInt(byte);
  }

  while (bigNumber > 0) {
    const remainder = Number(bigNumber % BigInt(58));
    encoded = alphabet[remainder] + encoded;
    bigNumber /= BigInt(58);
  }

  let leadingZeros = 0;
  for (const byte of bytes) {
    if (byte !== 0) {
      break;
    }
    leadingZeros++;
  }

  return "1".repeat(leadingZeros) + encoded;
};
