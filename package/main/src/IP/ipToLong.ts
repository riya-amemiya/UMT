import { ipToBinaryString } from "./ipToBinaryString";

/**
 * Converts an IPv4 address to a 32-bit number
 * @param {string} ip - IPv4 address to convert (e.g., "192.168.1.1")
 * @returns {number} 32-bit unsigned integer
 */
export const ipToLong = (ip: string): number =>
  Number.parseInt(ipToBinaryString(ip), 2) >>> 0;
