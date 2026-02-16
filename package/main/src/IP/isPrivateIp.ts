import { isInRange } from "./isInRange";

/**
 * Checks if an IP address is within private IP ranges
 * @param {string} ip - IP address to check (e.g., "192.168.1.1")
 * @returns {boolean} True if the IP is private, false otherwise
 */
export const isPrivateIp = (ip: string): boolean => {
  // Define private IP ranges with their CIDR notations
  const privateRanges = [
    { network: "10.0.0.0", cidr: 8 }, // Class A private network
    { network: "172.16.0.0", cidr: 12 }, // Class B private network
    { network: "192.168.0.0", cidr: 16 }, // Class C private network
  ];

  return privateRanges.some((range) =>
    isInRange(ip, range.network, range.cidr),
  );
};
