/**
 * IPアドレスをバイナリ文字列に変換します
 * @param {string} ip - 変換するIPアドレス
 * @returns {string} バイナリ文字列
 */
export const ipToBinaryString = (ip: string): string =>
  ip
    .split(".")
    .map((octet) => (+octet).toString(2).padStart(8, "0"))
    .join("");
