const TEXT_ENCODER = new TextEncoder();

/**
Encodes a string or Uint8Array to lowercase hexadecimal
@param {string | Uint8Array} input - The input to encode (strings are UTF-8 encoded first)
@returns {string} Lowercase hexadecimal string with two characters per byte
@example encodeHex("Hello"); // "48656c6c6f"
*/
export const encodeHex = (input: string | Uint8Array): string => {
  const bytes = typeof input === "string" ? TEXT_ENCODER.encode(input) : input;
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join(
    "",
  );
};
