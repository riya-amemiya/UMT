import { cidrToLong } from "./cidrToLong";
import { ipToLong } from "./ipToLong";
import { subnetMaskToCidr } from "./subnetMaskToCidr";

/**
 * ネットワークアドレスを取得します。
 * @param {string} ip - IPアドレス。
 * @param {string} subnetMask - サブネットマスク。
 * @returns {number} ネットワークアドレス。
 */
export const getNetworkAddress = (ip: string, subnetMask: string): number =>
  ipToLong(ip) & cidrToLong(subnetMaskToCidr(subnetMask));
