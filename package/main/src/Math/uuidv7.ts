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

  // Create a single 16-byte buffer for the UUID
  const bytes = new Uint8Array(16);

  // Fill the entire buffer with random data first.
  // This avoids allocating a separate 10-byte buffer for randomness.
  // We will overwrite the first 6 bytes with the timestamp.
  globalThis.crypto.getRandomValues(bytes);

  // Overwrite first 6 bytes with 48-bit timestamp (Big-Endian)
  // Note: We use division for the high 16 bits because JS bitwise operators (>>>)
  // truncate to 32 bits, which would corrupt the 48-bit timestamp.
  bytes[0] = (unixTsMs / 0x1_00_00_00_00_00) & 0xff;
  bytes[1] = (unixTsMs / 0x1_00_00_00_00) & 0xff;
  bytes[2] = (unixTsMs >>> 24) & 0xff;
  bytes[3] = (unixTsMs >>> 16) & 0xff;
  bytes[4] = (unixTsMs >>> 8) & 0xff;
  bytes[5] = unixTsMs & 0xff;

  // Version 7 (0x70) + 4 bits from random data (already in bytes[6])
  bytes[6] = 0x70 | (bytes[6] & 0x0f);

  // Variant 2 (0x80) + 6 bits from random data (already in bytes[8])
  bytes[8] = 0x80 | (bytes[8] & 0x3f);

  // Convert to string (unrolled for performance)
  // Format: 8-4-4-4-12 hex digits (xxxxxxxx-xxxx-7xxx-8xxx-xxxxxxxxxxxx)
  return (
    DIGITS[bytes[0] >>> 4] +
    DIGITS[bytes[0] & 0xf] +
    DIGITS[bytes[1] >>> 4] +
    DIGITS[bytes[1] & 0xf] +
    DIGITS[bytes[2] >>> 4] +
    DIGITS[bytes[2] & 0xf] +
    DIGITS[bytes[3] >>> 4] +
    DIGITS[bytes[3] & 0xf] +
    "-" +
    DIGITS[bytes[4] >>> 4] +
    DIGITS[bytes[4] & 0xf] +
    DIGITS[bytes[5] >>> 4] +
    DIGITS[bytes[5] & 0xf] +
    "-" +
    DIGITS[bytes[6] >>> 4] +
    DIGITS[bytes[6] & 0xf] +
    DIGITS[bytes[7] >>> 4] +
    DIGITS[bytes[7] & 0xf] +
    "-" +
    DIGITS[bytes[8] >>> 4] +
    DIGITS[bytes[8] & 0xf] +
    DIGITS[bytes[9] >>> 4] +
    DIGITS[bytes[9] & 0xf] +
    "-" +
    DIGITS[bytes[10] >>> 4] +
    DIGITS[bytes[10] & 0xf] +
    DIGITS[bytes[11] >>> 4] +
    DIGITS[bytes[11] & 0xf] +
    DIGITS[bytes[12] >>> 4] +
    DIGITS[bytes[12] & 0xf] +
    DIGITS[bytes[13] >>> 4] +
    DIGITS[bytes[13] & 0xf] +
    DIGITS[bytes[14] >>> 4] +
    DIGITS[bytes[14] & 0xf] +
    DIGITS[bytes[15] >>> 4] +
    DIGITS[bytes[15] & 0xf]
  );
};
