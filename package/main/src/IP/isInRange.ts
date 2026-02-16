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

  // Special cases
  if (cidr === 0) {
    return true; // All IPs are in range
  }
  if (cidr === 32) {
    return remoteLong === networkLong; // Exact match required
  }

  // Normal case
  const shift = 32 - cidr;
  const mask = (0xff_ff_ff_ff >>> 0) << shift;
  return (remoteLong & mask) === (networkLong & mask);
};
