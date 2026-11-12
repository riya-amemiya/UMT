/**
Decodes a hexadecimal string to Uint8Array
@param {string} input - Hexadecimal string (lowercase or uppercase, two characters per byte)
@returns {Uint8Array} Decoded bytes
@example decodeHex("48656c6c6f"); // Uint8Array for "Hello"
*/
export const decodeHex = (input: string): Uint8Array =>
  Uint8Array.from({ length: input.length >> 1 }, (_, index) =>
    Number.parseInt(input.slice(index * 2, index * 2 + 2), 16),
  );
