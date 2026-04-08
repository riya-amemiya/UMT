/**
 * Converts CIDR notation to a subnet mask number
 * @param {number} cidr - CIDR notation (0-32)
 * @returns {number} Subnet mask as a 32-bit number
 * @throws {Error} If the CIDR value is not an integer between 0 and 32
 */
export const cidrToLong = (cidr: number): number => {
  // Security: Reject non-integer, NaN, negative, or out-of-range values.
  // Without this check, NaN or negative inputs silently produce mask 0
  // (matching all IPs), which could bypass IP-based access controls
  // in downstream functions like isInRange or isPrivateIp.
  if (!Number.isInteger(cidr) || cidr < 0 || cidr > 32) {
    throw new Error("cidrToLong: CIDR must be an integer between 0 and 32");
  }
  return Number.parseInt("1".repeat(cidr).padEnd(32, "0"), 2) >>> 0;
};
