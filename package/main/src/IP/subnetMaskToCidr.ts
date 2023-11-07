import { ipToBinaryString } from "./ipToBinaryString";

/**
 * サブネットマスクからCIDRを取得します
 * @param {string} subnetMask - サブネットマスク
 * @returns {number} CIDR
 */
export const subnetMaskToCidr = (subnetMask: string): number =>
  ipToBinaryString(subnetMask).match(/1/g)?.length || 0;
