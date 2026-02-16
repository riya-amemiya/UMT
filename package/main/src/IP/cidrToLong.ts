/**
 * Converts CIDR notation to a subnet mask number
 * @param {number} cidr - CIDR notation (0-32)
 * @returns {number} Subnet mask as a 32-bit number
 */
export const cidrToLong = (cidr: number): number => {
  return Number.parseInt("1".repeat(cidr).padEnd(32, "0"), 2) >>> 0;
};
