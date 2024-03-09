/**
 * IPクラスを取得します
 * @param {string} ip - IPアドレス
 * @returns {string} IPクラス（A, B, C, D, Eまたは空文字列）
 */
export const getIpClass = (ip: string): string => {
  if (!ip) {
    return "";
  }
  // IPアドレスの最初のオクテットを取得
  const firstOctet = Number.parseInt(ip.split(".")[0]);

  if (firstOctet < 128) {
    return "A";
  }
  if (firstOctet < 192) {
    return "B";
  }
  if (firstOctet < 224) {
    return "C";
  }
  if (firstOctet < 240) {
    return "D";
  }
  if (firstOctet < 256) {
    return "E";
  }
  return "";
};
