import { unwrap } from "@/Tool/unwrap";

const FULL_WIDTH_CHARS = Array.from({ length: 128 }, (_, code) => {
  const isAlphanumeric =
    (code >= 48 && code <= 57) ||
    (code >= 65 && code <= 90) ||
    (code >= 97 && code <= 122);
  return String.fromCodePoint(isAlphanumeric ? code + 0xfe_e0 : code);
});

/**
Convert half-width alphanumeric characters to full-width characters.
Inverse of `toHalfWidth`; only digits, uppercase and lowercase ASCII
letters are converted.
@param {string} string_ - String to convert
@returns {string} - Converted string
@example
toFullWidth("Abc123"); // "Ａｂｃ１２３"
*/
export const toFullWidth = (string_: string): string => {
  let result = "";
  for (const character of string_) {
    const code = unwrap(
      character.codePointAt(0),
      "panic: invalid toFullWidth string",
    );
    result += code < 128 ? FULL_WIDTH_CHARS[code] : character;
  }
  return result;
};
