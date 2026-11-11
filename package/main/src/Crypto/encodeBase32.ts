import { BASE32_ALPHABET } from "./constants";

import { unwrap } from "@/Tool/unwrap";

const TEXT_ENCODER = new TextEncoder();
const TEXT_DECODER = new TextDecoder();
const BASE32_ALPHABET_CODES = new Uint8Array(32);
const BASE32_PADDING_CHAR_CODE = unwrap(
  "=".codePointAt(0),
  "panic: invalid base32 padding",
);

for (let index = 0; index < BASE32_ALPHABET.length; index++) {
  BASE32_ALPHABET_CODES[index] = unwrap(
    BASE32_ALPHABET.codePointAt(index),
    "panic: invalid base32 alphabet",
  );
}

/**
Encodes a string or Uint8Array to Base32 format
@param {string | Uint8Array} input - The input to encode
@returns {string} Base32 encoded string
@example encodeBase32("Hello"); // "JBSWY3DP"
*/
export const encodeBase32 = (input: string | Uint8Array): string => {
  const bytes = typeof input === "string" ? TEXT_ENCODER.encode(input) : input;

  if (bytes.length === 0) {
    return "";
  }

  const charCount = Math.ceil((bytes.length * 8) / 5);
  const paddingLength = (8 - (charCount % 8)) % 8;
  const output = new Uint8Array(charCount + paddingLength);
  let offset = 0;
  let buffer = 0;
  let bufferLength = 0;

  for (const byte of bytes) {
    buffer = (buffer << 8) | byte;
    bufferLength += 8;

    while (bufferLength >= 5) {
      bufferLength -= 5;
      output[offset++] = BASE32_ALPHABET_CODES[(buffer >> bufferLength) & 0x1f];
    }
  }

  if (bufferLength > 0) {
    output[offset++] =
      BASE32_ALPHABET_CODES[(buffer << (5 - bufferLength)) & 0x1f];
  }

  output.fill(BASE32_PADDING_CHAR_CODE, offset);
  return TEXT_DECODER.decode(output);
};
