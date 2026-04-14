/**
 * Converts a 32-bit number to an IPv4 address
 * @param {number} long - 32-bit unsigned integer to convert (0 to 4294967295)
 * @returns {string} IPv4 address (e.g., "192.168.1.1")
 */
export const longToIp = (long: number): string => {
  return `${(long >>> 24) & 0xff}.${(long >>> 16) & 0xff}.${(long >>> 8) & 0xff}.${long & 0xff}`;
};
