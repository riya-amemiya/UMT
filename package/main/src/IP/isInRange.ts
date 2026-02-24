import { cidrToLong } from "./cidrToLong";
import { ipToLong } from "./ipToLong";

/**
 * Checks if an IP address is within a specified network range
 * @param {string} remoteIp - IP address to check (e.g., "192.168.1.1")
 * @param {string} networkIp - Network IP address (e.g., "192.168.0.0")
 * @param {number} cidr - CIDR notation (0-32)
 * @returns {boolean} True if the IP is in range, false otherwise
 */
export const isInRange = (
  remoteIp: string,
  networkIp: string,
  cidr: number,
): boolean => {
  const remoteLong = ipToLong(remoteIp);
  const networkLong = ipToLong(networkIp);
  const mask = cidrToLong(cidr);
  return (remoteLong & mask) === (networkLong & mask);
};
