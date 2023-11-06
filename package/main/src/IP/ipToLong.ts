import { ipToBinaryString } from "./ipToBinaryString";

/**
 * IPアドレスを32ビットの数値に変換します。
 * @param {string} ip - 変換するIPアドレス。
 * @returns {number} 32ビットの数値。
 */
export const ipToLong = (ip: string): number =>
  parseInt(ipToBinaryString(ip), 2);
