import { unwrap } from "@/Tool/unwrap";

const HEX_LOWER = "0123456789abcdef";
const HEX_UPPER = "ABCDEF";
const HEX_DECODE_TABLE = new Uint8Array(256);

for (let index = 0; index < HEX_LOWER.length; index++) {
  HEX_DECODE_TABLE[
    unwrap(HEX_LOWER.codePointAt(index), "panic: invalid hex alphabet")
  ] = index;
}
for (let index = 0; index < HEX_UPPER.length; index++) {
  HEX_DECODE_TABLE[
    unwrap(HEX_UPPER.codePointAt(index), "panic: invalid hex alphabet")
  ] = index + 10;
}

/**
Decodes a hexadecimal string to Uint8Array
@param {string} input - Hexadecimal string (lowercase or uppercase, two characters per byte)
@returns {Uint8Array} Decoded bytes
@example decodeHex("48656c6c6f"); // Uint8Array for "Hello"
*/
export const decodeHex = (input: string): Uint8Array => {
  const byteLength = input.length >> 1;
  const output = new Uint8Array(byteLength);

  for (let index = 0; index < byteLength; index++) {
    const high = unwrap(
      input.codePointAt(index * 2),
      "panic: invalid hex string",
    );
    const low = unwrap(
      input.codePointAt(index * 2 + 1),
      "panic: invalid hex string",
    );
    output[index] =
      ((HEX_DECODE_TABLE[high] ?? 0) << 4) | (HEX_DECODE_TABLE[low] ?? 0);
  }

  return output;
};
