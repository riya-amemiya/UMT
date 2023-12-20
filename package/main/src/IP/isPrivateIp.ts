import { ipToLong } from "./ipToLong";
import { isInRange } from "./isInRange";

/**
 * IPアドレスがプライベート範囲にあるかどうかを判定します
 * @param {string} ip - 判定するIPアドレス
 * @returns {boolean} プライベート範囲にあればtrue
 */
export const isPrivateIp = (ip: string): boolean => {
  // プライベートIP範囲の定義
  const privateRanges = [
    { start: "10.0.0.0", end: "10.255.255.255" },
    { start: "172.16.0.0", end: "172.31.255.255" },
    { start: "192.168.0.0", end: "192.168.255.255" },
  ];

  return privateRanges.some((range) =>
    isInRange(ip, range.start, ipToLong(range.end)),
  );
};
