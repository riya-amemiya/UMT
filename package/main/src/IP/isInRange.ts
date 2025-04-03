import { ipToLong } from "./ipToLong";

/**
 * Checks if an IP address is within a specified network range
 * @param {string} remoteIp - IP address to check (e.g., "192.168.1.1")
 * @param {string} networkIp - Network IP address (e.g., "192.168.0.0")
 * @param {number} cidr - CIDR notation (0-32)
 * @returns {boolean} True if the IP is in range, false otherwise
 * @throws {Error} If any parameter is invalid
 */
export const isInRange = (
  remoteIp: string,
  networkIp: string,
  cidr: number,
): boolean => {
  if (!remoteIp) {
    throw new Error("Remote IP address is required");
  }
  if (!networkIp) {
    throw new Error("Network IP address is required");
  }

  if (!Number.isInteger(cidr) || cidr < 0 || cidr > 32) {
    throw new Error("CIDR must be an integer between 0 and 32");
  }

  try {
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
  } catch (error) {
    throw new Error(`Invalid IP address format: ${String(error)}`);
  }
};
