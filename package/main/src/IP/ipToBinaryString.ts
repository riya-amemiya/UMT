/**
 * Converts an IPv4 address to its binary string representation
 * @param {string} ip - IPv4 address (e.g., "192.168.1.1")
 * @returns {string} Binary string representation (32 bits)
 */
export const ipToBinaryString = (ip: string): string => {
  return ip
    .split(".")
    .map((octet) => Number.parseInt(octet, 10).toString(2).padStart(8, "0"))
    .join("");
};
