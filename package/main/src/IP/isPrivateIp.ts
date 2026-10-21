import { cidrToLong } from "./cidrToLong";
import { ipToLong } from "./ipToLong";

const PRIVATE_RANGES: [number, number][] = [
  [ipToLong("10.0.0.0"), cidrToLong(8)],
  [ipToLong("172.16.0.0"), cidrToLong(12)],
  [ipToLong("192.168.0.0"), cidrToLong(16)],
];

/**
Checks if an IP address is within private IP ranges
@param {string} ip - IP address to check (e.g., "192.168.1.1")
@returns {boolean} True if the IP is private, false otherwise
*/
export const isPrivateIp = (ip: string): boolean => {
  const remoteLong = ipToLong(ip);
  return PRIVATE_RANGES.some(
    ([network, mask]) => (remoteLong & mask) === (network & mask),
  );
};
