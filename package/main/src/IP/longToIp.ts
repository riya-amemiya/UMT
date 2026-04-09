/**
 * Converts a 32-bit number to an IPv4 address
 * @param {number} long - 32-bit unsigned integer to convert (0 to 4294967295)
 * @returns {string} IPv4 address (e.g., "192.168.1.1")
 * @throws {Error} If the input is not a valid 32-bit unsigned integer
 */
export const longToIp = (long: number): string => {
  // Security: Validate input bounds to prevent generation of invalid IP addresses.
  // Without validation, negative numbers, NaN, Infinity, or values > 2^32-1 produce
  // garbage output that could bypass IP-based access controls (e.g. isInRange,
  // isPrivateIp) when used downstream.
  if (!Number.isInteger(long) || long < 0 || long > 0xff_ff_ff_ff) {
    throw new Error(
      "longToIp: input must be a 32-bit unsigned integer (0 to 4294967295)",
    );
  }

  // Extract each octet using bitwise operations instead of string conversion.
  // This avoids creating a 32-char binary string, slicing, and parsing it back.
  return `${(long >>> 24) & 0xff}.${(long >>> 16) & 0xff}.${(long >>> 8) & 0xff}.${long & 0xff}`;
};
