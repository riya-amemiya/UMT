import { cidrToLong } from "./cidrToLong";
import { longToIp } from "./longToIp";

/**
 * CIDRからサブネットマスクを取得します
 * @param {number} cidr - CIDR
 * @returns {string} サブネットマスク
 */
export const cidrToSubnetMask = (cidr: number): string =>
  longToIp(cidrToLong(cidr));
