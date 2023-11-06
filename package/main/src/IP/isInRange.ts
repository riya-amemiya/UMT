import { ipToLong } from "./ipToLong";

/**
 * 指定のIPアドレスが範囲内かどうかを判定します。
 * @param {string} remoteIp - リモートIPアドレス。
 * @param {string} networkIp - ネットワークIPアドレス。
 * @param {number} cidr - CIDR。
 * @returns {boolean} 範囲内であればtrue。
 */
export const isInRange = (
  remoteIp: string,
  networkIp: string,
  cidr: number,
): boolean => {
  const shift = 32 - cidr;
  return ipToLong(remoteIp) >>> shift === ipToLong(networkIp) >>> shift;
};
