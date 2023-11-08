import { cidrToLong } from "./cidrToLong";
import { ipToLong } from "./ipToLong";
import { subnetMaskToCidr } from "./subnetMaskToCidr";

/**
 * ネットワークアドレスを取得します
 * @param {string} ip - IPアドレス
 * @param {string} subnetMask - サブネットマスク
 * @returns {number} ネットワークアドレス
 */
export const getNetworkAddress = (ip: string, subnetMask: string): number => {
  const networkAddress =
    ipToLong(ip) & cidrToLong(subnetMaskToCidr(subnetMask));
  return networkAddress >>> 0; // 符号なし32ビット整数に変換
};
