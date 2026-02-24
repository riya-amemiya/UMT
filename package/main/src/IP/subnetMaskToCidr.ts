import { ipToBinaryString } from "./ipToBinaryString";

/**
 * Converts a subnet mask to CIDR notation
 * @param {string} subnetMask - IPv4 subnet mask (e.g., "255.255.255.0")
 * @returns {number} CIDR notation (0-32)
 */
export const subnetMaskToCidr = (subnetMask: string): number => {
  const binaryString = ipToBinaryString(subnetMask);

  // Count 1s
  const cidr = binaryString.match(/1/g)?.length ?? 0;
  return cidr;
};
