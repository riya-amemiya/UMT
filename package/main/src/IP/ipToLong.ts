/**
 * Converts an IPv4 address to a 32-bit number
 * @param {string} ip - IPv4 address to convert (e.g., "192.168.1.1")
 * @returns {number} 32-bit unsigned integer
 */
export const ipToLong = (ip: string): number => {
  const parts = ip.split(".");
  let result = 0;
  for (const part of parts) {
    result = ((result << 8) | Number.parseInt(part, 10)) >>> 0;
  }
  return result;
};
