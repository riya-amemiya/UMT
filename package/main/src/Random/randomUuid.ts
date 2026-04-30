const DIGITS = "0123456789abcdef";

const bytesToUuid = (bytes: Uint8Array): string => {
  let result = "";
  for (let index = 0; index < 16; index += 1) {
    if (index === 4 || index === 6 || index === 8 || index === 10) {
      result += "-";
    }
    const byte = bytes[index];
    result += DIGITS[byte >>> 4] + DIGITS[byte & 0xf];
  }
  return result;
};

/**
 * Generates a UUID v4 string. Uses `crypto.randomUUID` when available,
 * otherwise falls back to `crypto.getRandomValues`.
 *
 * @returns {string} A UUID v4 in canonical 8-4-4-4-12 hex form
 * @example
 * randomUUID(); // e.g. "1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed"
 */
export const randomUUID = (): string => {
  if (typeof globalThis.crypto?.randomUUID === "function") {
    return globalThis.crypto.randomUUID();
  }
  const bytes = new Uint8Array(16);
  globalThis.crypto.getRandomValues(bytes);
  bytes[6] = (bytes[6] & 0x0f) | 0x40;
  bytes[8] = (bytes[8] & 0x3f) | 0x80;
  return bytesToUuid(bytes);
};
