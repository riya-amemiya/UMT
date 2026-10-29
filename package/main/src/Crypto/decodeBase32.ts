import { BASE32_ALPHABET } from "./constants";

import { unwrap } from "@/Tool/unwrap";

const BASE32_DECODE_TABLE = new Uint8Array(256);
const BASE32_PADDING_CHAR_CODE = unwrap(
  "=".codePointAt(0),
  "panic: invalid base32 padding",
);

for (let index = 0; index < BASE32_ALPHABET.length; index++) {
  BASE32_DECODE_TABLE[
    unwrap(BASE32_ALPHABET.codePointAt(index), "panic: invalid base32 alphabet")
  ] = index;
}

/**
Decodes a Base32 string to Uint8Array
@param {string} input - Base32 encoded string
@returns {Uint8Array} Decoded bytes
@example decodeBase32("JBSWY3DP"); // Uint8Array for "Hello"
*/
export const decodeBase32 = (input: string): Uint8Array => {
  const result: number[] = [];
  let buffer = 0;
  let bufferLength = 0;

  for (let index = 0; index < input.length; index++) {
    const code = unwrap(
      input.codePointAt(index),
      "panic: invalid base32 string",
    );
    if (code === BASE32_PADDING_CHAR_CODE) {
      continue;
    }
    if (code > 0xff_ff) {
      index++;
    }

    const value = BASE32_DECODE_TABLE[code] ?? 0;
    buffer = (buffer << 5) | value;
    bufferLength += 5;

    if (!(bufferLength >= 8)) {
      continue;
    }

    bufferLength -= 8;
    result.push((buffer >> bufferLength) & 0xff);
  }

  return new Uint8Array(result);
};
