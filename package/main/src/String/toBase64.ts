/**
 * 文字列をBase64に変換する
 * @param char Base64に変換する文字列
 * @returns Base64に変換された文字列
 */
export const toBase64 = (char: string): string =>
  btoa(
    encodeURIComponent(char).replaceAll(/%([\dA-F]{2})/g, (_, p1) =>
      String.fromCodePoint(parseInt(p1, 16)),
    ),
  );
