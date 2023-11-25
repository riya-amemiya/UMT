/**
 * 全角文字を半角文字に変換する
 * @param {string} str - 変換する文字列
 * @returns {string} - 変換された文字列
 */
export const toHalfWidth = (string_: string): string =>
  string_.replaceAll(/[０-９Ａ-Ｚａ-ｚ]/g, (s) => {
    // eslint-disable-next-line unicorn/prefer-code-point
    const code = s.charCodeAt(0);
    return String.fromCodePoint(code - 0xfe_e0);
  });
