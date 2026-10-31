import { unwrap } from "@/Tool/unwrap";

const HALF_WIDTH_CHARS = Array.from({ length: 96 }, (_, index) => {
  const code = 0xff_00 + index;
  const isAlphanumeric =
    (code >= 0xff_10 && code <= 0xff_19) ||
    (code >= 0xff_21 && code <= 0xff_3a) ||
    (code >= 0xff_41 && code <= 0xff_5a);
  return String.fromCodePoint(isAlphanumeric ? code - 0xfe_e0 : code);
});

/**
Convert full-width characters to half-width characters
@param {string} str - String to convert
@returns {string} - Converted string
*/
export const toHalfWidth = (string_: string): string => {
  let result = "";
  for (const character of string_) {
    const code = unwrap(
      character.codePointAt(0),
      "panic: invalid toHalfWidth string",
    );
    result +=
      code >= 0xff_00 && code < 0xff_60
        ? HALF_WIDTH_CHARS[code - 0xff_00]
        : character;
  }
  return result;
};
