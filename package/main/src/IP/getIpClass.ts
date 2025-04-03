/**
 * Gets the IP address class (A, B, C, D, or E)
 * @param {string} ip - IPv4 address
 * @returns {string} IP class ('A', 'B', 'C', 'D', 'E', or empty string for invalid IP)
 */
export const getIpClass = (ip: string): string => {
  if (!ip) {
    return "";
  }

  // Validate IP format
  const parts = ip.split(".");
  if (parts.length !== 4) {
    return "";
  }

  const firstOctet = Number.parseInt(parts[0]);
  if (Number.isNaN(firstOctet) || firstOctet < 0 || firstOctet > 255) {
    return "";
  }

  // Check each class range
  switch (true) {
    case firstOctet === 0: {
      return "";
    } // Reserved
    case firstOctet < 128: {
      return "A";
    } // 1-127
    case firstOctet < 192: {
      return "B";
    } // 128-191
    case firstOctet < 224: {
      return "C";
    } // 192-223
    case firstOctet < 240: {
      return "D";
    } // 224-239
    default: {
      return "E";
    } // 240-255
  }
};
