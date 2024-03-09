/**
 * 32ビットの数値をIPアドレスに変換します
 * @param {number} long - 変換する32ビットの数値
 * @returns {string} IPアドレス
 */
export const longToIp = (long: number): string => {
  const binary = long.toString(2).padStart(32, "0");
  return Array.from({ length: 4 }, (_, index) =>
    Number.parseInt(binary.slice(index * 8, (index + 1) * 8), 2),
  ).join(".");
};
