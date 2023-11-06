import { cidrToLong } from "./cidrToLong";
import { ipToLong } from "./ipToLong";
import { subnetMaskToCidr } from "./subnetMaskToCidr";

/**
 * ブロードキャストアドレスを取得します
 * @param {string} ip - IPアドレス
 * @param {string} subnetMask - サブネットマスク
 * @returns {number} ブロードキャストアドレス
 */
export const getBroadcastAddress = (ip: string, subnetMask: string): number =>
  ipToLong(ip) | ~cidrToLong(subnetMaskToCidr(subnetMask));
