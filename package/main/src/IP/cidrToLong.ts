/**
 * Converts CIDR notation to a subnet mask number
 * @param {number} cidr - CIDR notation (0-32)
 * @returns {number} Subnet mask as a 32-bit number
 * @throws {Error} If CIDR is not between 0 and 32
 */
export const cidrToLong = (cidr: number): number => {
  if (!Number.isInteger(cidr) || cidr < 0 || cidr > 32) {
    throw new Error("CIDR must be an integer between 0 and 32");
  }
  return Number.parseInt("1".repeat(cidr).padEnd(32, "0"), 2) >>> 0;
};
