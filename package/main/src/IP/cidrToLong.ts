/**
 * Converts CIDR notation to a subnet mask number
 * @param {number} cidr - CIDR notation (0-32)
 * @returns {number} Subnet mask as a 32-bit number
 */
export const cidrToLong = (cidr: number): number => {
  return cidr === 0 ? 0 : (~0 << (32 - cidr)) >>> 0;
};
