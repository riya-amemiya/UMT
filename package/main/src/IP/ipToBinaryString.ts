/**
 * Converts an IPv4 address to its binary string representation
 * @param {string} ip - IPv4 address (e.g., "192.168.1.1")
 * @returns {string} Binary string representation (32 bits)
 * @throws {Error} If the IP address format is invalid
 */
export const ipToBinaryString = (ip: string): string => {
  if (typeof ip !== "string" || ip.length === 0) {
    throw new Error("ipToBinaryString: input must be a non-empty string");
  }

  const parts = ip.split(".");
  if (parts.length !== 4) {
    throw new Error("ipToBinaryString: expected 4 octets separated by dots");
  }

  return parts
    .map((part) => {
      if (!/^\d{1,3}$/.test(part)) {
        throw new Error(`ipToBinaryString: invalid octet "${part}"`);
      }
      const value = Number.parseInt(part, 10);
      if (value < 0 || value > 255) {
        throw new Error(`ipToBinaryString: octet ${value} out of range 0-255`);
      }
      return value.toString(2).padStart(8, "0");
    })
    .join("");
};
