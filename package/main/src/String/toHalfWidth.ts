/**
 * Convert full-width characters to half-width characters
 * @param {string} str - String to convert
 * @returns {string} - Converted string
 */
export const toHalfWidth = (string_: string): string =>
  string_.replaceAll(/[０-９Ａ-Ｚａ-ｚ]/g, (s) => {
    const code = s.codePointAt(0);
    // biome-ignore lint/style/noNonNullAssertion: code is not null
    return String.fromCodePoint(code! - 0xfe_e0);
  });
