import { cidrToLong } from "./cidrToLong";
import { longToIp } from "./longToIp";

/**
 * Converts CIDR notation to a subnet mask
 * @param {number} cidr - CIDR notation (0-32)
 * @returns {string} Subnet mask in IPv4 format (e.g., "255.255.255.0")
 * @throws {Error} If CIDR is not between 0 and 32
 */
export const cidrToSubnetMask = (cidr: number): string =>
  longToIp(cidrToLong(cidr));
