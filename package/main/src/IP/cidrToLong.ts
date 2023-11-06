/**
 * CIDRからサブネットマスクの数値を取得します。
 * @param {number} cidr - CIDR。
 * @returns {number} サブネットマスクの数値。
 */
export const cidrToLong = (cidr: number): number =>
  parseInt("1".repeat(cidr).padEnd(32, "0"), 2);
