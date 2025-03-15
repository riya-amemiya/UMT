/**
 * Converts a 32-bit number to an IPv4 address
 * @param {number} long - 32-bit unsigned integer to convert
 * @returns {string} IPv4 address (e.g., "192.168.1.1")
 * @throws {Error} If input is not a valid 32-bit unsigned integer
 */
export const longToIp = (long: number): string => {
  // Validate input
  if (
    !Number.isFinite(long) ||
    long < 0 ||
    long > 0xff_ff_ff_ff ||
    !Number.isInteger(long)
  ) {
    throw new Error("Input must be a valid 32-bit unsigned integer");
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
