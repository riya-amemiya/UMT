import { cidrToLong } from "./cidrToLong";
import { ipToLong } from "./ipToLong";
import { subnetMaskToCidr } from "./subnetMaskToCidr";

/**
 * Calculates the network address from an IP address and subnet mask
 * @param {string} ip - IPv4 address (e.g., "192.168.1.1")
 * @param {string} subnetMask - Subnet mask (e.g., "255.255.255.0")
 * @returns {number} Network address as a 32-bit unsigned integer
 */
export const getNetworkAddress = (ip: string, subnetMask: string): number => {
  const networkAddress =
    ipToLong(ip) & cidrToLong(subnetMaskToCidr(subnetMask));
  return networkAddress >>> 0; // Convert to unsigned 32-bit integer
};
