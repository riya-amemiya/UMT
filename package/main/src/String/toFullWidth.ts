/**
 * Convert half-width alphanumeric characters to full-width characters.
 * Inverse of `toHalfWidth`; only digits, uppercase and lowercase ASCII
 * letters are converted.
 * @param {string} string_ - String to convert
 * @returns {string} - Converted string
 * @example
 * toFullWidth("Abc123"); // "Ａｂｃ１２３"
 */
export const toFullWidth = (string_: string): string =>
  string_.replaceAll(/[\dA-Za-z]/g, (s) => {
    const code = s.codePointAt(0);
    // biome-ignore lint/style/noNonNullAssertion: code is not null
    return String.fromCodePoint(code! + 0xfe_e0);
  });
