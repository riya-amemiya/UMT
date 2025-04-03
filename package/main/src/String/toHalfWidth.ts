/**
 * Convert full-width characters to half-width characters
 * @param {string} str - String to convert
 * @returns {string} - Converted string
 */
export const toHalfWidth = (string_: string): string =>
  string_.replaceAll(/[０-９Ａ-Ｚａ-ｚ]/g, (s) => {
    // eslint-disable-next-line unicorn/prefer-code-point
    const code = s.charCodeAt(0);
    return String.fromCodePoint(code - 0xfe_e0);
  });
