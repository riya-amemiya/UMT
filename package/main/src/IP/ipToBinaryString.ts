/**
 * Converts an IPv4 address to its binary string representation
 * @param {string} ip - IPv4 address (e.g., "192.168.1.1")
 * @returns {string} Binary string representation (32 bits)
 * @throws {Error} If IP address is invalid
 */
export const ipToBinaryString = (ip: string): string => {
  if (!ip) {
    throw new Error("IP address is required");
  }

  // Check for invalid characters
  if (/[^0-9.]/.test(ip)) {
    throw new Error("Invalid IP address format");
  }

  const parts = ip.split(".");
  if (parts.length !== 4) {
    throw new Error("Invalid IP address format");
  }

  // Validate each octet
  for (const octet of parts) {
    // Check for empty octet or leading zeros
    if (!octet || (octet.length > 1 && octet[0] === "0")) {
      throw new Error("Invalid IP address format");
    }

    const number = Number.parseInt(octet);
    if (Number.isNaN(number) || number < 0 || number > 255) {
      throw new Error("Invalid IP address format");
    }
  }

  // Convert to binary
  return parts
    .map((octet) => Number.parseInt(octet).toString(2).padStart(8, "0"))
    .join("");
};
