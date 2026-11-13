import { unwrap } from "@/Tool/unwrap";

const TEXT_ENCODER = new TextEncoder();
const TEXT_DECODER = new TextDecoder();
const HEX_ALPHABET = "0123456789abcdef";
const HEX_DIGITS = new Uint8Array(16);

for (let index = 0; index < HEX_ALPHABET.length; index++) {
  HEX_DIGITS[index] = unwrap(
    HEX_ALPHABET.codePointAt(index),
    "panic: invalid hex alphabet",
  );
}

/**
Encodes a string or Uint8Array to lowercase hexadecimal
@param {string | Uint8Array} input - The input to encode (strings are UTF-8 encoded first)
@returns {string} Lowercase hexadecimal string with two characters per byte
@example encodeHex("Hello"); // "48656c6c6f"
*/
export const encodeHex = (input: string | Uint8Array): string => {
  const bytes = typeof input === "string" ? TEXT_ENCODER.encode(input) : input;

  if (bytes.length === 0) {
    return "";
  }

  const output = new Uint8Array(bytes.length * 2);
  let offset = 0;
  for (const byte of bytes) {
    output[offset++] = HEX_DIGITS[byte >> 4];
    output[offset++] = HEX_DIGITS[byte & 0x0f];
  }
  return TEXT_DECODER.decode(output);
};
