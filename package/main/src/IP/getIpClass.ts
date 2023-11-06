import { ipToLong } from "./ipToLong";

/**
 * IPクラスを取得します
 * @param {string} ip - IPアドレス
 * @returns {string} IPクラス（A, B, C, D, Eまたは空文字列）
 */
export const getIpClass = (ip: string): string => {
  const long = ipToLong(ip);
  if (long >= ipToLong("0.0.0.0") && long <= ipToLong("127.255.255.255")) {
    return "A";
  }
  if (long >= ipToLong("128.0.0.0") && long <= ipToLong("191.255.255.255")) {
    return "B";
  }
  if (long >= ipToLong("192.0.0.0") && long <= ipToLong("223.255.255.255")) {
    return "C";
  }
  if (long >= ipToLong("224.0.0.0") && long <= ipToLong("239.255.255.255")) {
    return "D";
  }
  if (long >= ipToLong("240.0.0.0") && long <= ipToLong("255.255.255.255")) {
    return "E";
  }
  return "";
};
