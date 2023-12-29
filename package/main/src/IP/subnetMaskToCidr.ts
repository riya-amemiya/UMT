import { isValueNaN } from "@/Math/isValueNaN";
import { ipToBinaryString } from "./ipToBinaryString";

/**
 * サブネットマスクからCIDRを取得します
 * @param {string} subnetMask - サブネットマスク
 * @returns {number} CIDR
 */
export const subnetMaskToCidr = (subnetMask: string): number => {
  // サブネットマスクが有効なIPアドレス形式かどうかをチェック
  const octets = subnetMask.split(".").map(Number);
  if (
    octets.length !== 4 ||
    octets.some((octet) => isValueNaN(octet, true) || octet < 0 || octet > 255)
  ) {
    return 0;
  }
  return (ipToBinaryString(subnetMask).match(/1/g) as RegExpMatchArray)
    .length as number;
};
