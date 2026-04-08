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

  // Convert to binary string and ensure 32-bit length
  const binary = long.toString(2).padStart(32, "0");

  // Split into octets and convert to decimal
  const octets = Array.from({ length: 4 }, (_, index) =>
    Number.parseInt(binary.slice(index * 8, (index + 1) * 8), 2),
  );

  // Join octets with dots
  return octets.join(".");
};
