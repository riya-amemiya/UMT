/**
 * Generates a UUID v7 (Universally Unique Identifier version 7)
 * @returns {string} A UUID v7 string in the format xxxxxxxx-xxxx-7xxx-8xxx-xxxxxxxxxxxx
 * @example
 * const id = uuidv7(); // e.g. "018d6e78-e1e5-7c3c-8bf9-ae5942f2ba1c"
 * @description
 * UUID v7 is time-ordered and contains:
 * - 48 bits of Unix timestamp in milliseconds
 * - 74 bits of random data
 * - 4 bits of version (7)
 * - 2 bits of variant (2)
 * This implementation follows the UUID v7 draft specification.
 */
export const uuidv7 = (): string => {
  const DIGITS = "0123456789abcdef";
  const unixTsMs = Date.now();

  const bytes = new Uint8Array(16);
  for (let index = 0; index < 6; index++) {
    bytes[index] = (unixTsMs >>> ((5 - index) * 8)) & 0xff;
  }

  // Generate 10 random bytes for the rest
  const randomBytes = new Uint8Array(10);
  globalThis.crypto.getRandomValues(randomBytes);

  // Version 7 (0x70) + 4 bits from randomBytes[0]
  bytes[6] = 0x70 | (randomBytes[0] & 0x0f);
  // Random byte
  bytes[7] = randomBytes[1];
  // Variant 2 (0x80) + 6 bits from randomBytes[2]
  bytes[8] = 0x80 | (randomBytes[2] & 0x3f);
  // Remaining random bytes
  bytes.set(randomBytes.subarray(3), 9);

  let uuid = "";
  for (const [index, byte] of bytes.entries()) {
    uuid += DIGITS[byte >>> 4] + DIGITS[byte & 0xf];
    if (index === 3 || index === 5 || index === 7 || index === 9) {
      uuid += "-";
    }
  }

  return uuid;
};
