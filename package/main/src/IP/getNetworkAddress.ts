import { cidrToLong } from "./cidrToLong";
import { ipToLong } from "./ipToLong";
import { subnetMaskToCidr } from "./subnetMaskToCidr";

/**
 * Calculates the network address from an IP address and subnet mask
 * @param {string} ip - IPv4 address (e.g., "192.168.1.1")
 * @param {string} subnetMask - Subnet mask (e.g., "255.255.255.0")
 * @returns {number} Network address as a 32-bit unsigned integer
 * @throws {Error} If IP address or subnet mask is invalid
 */
export const getNetworkAddress = (ip: string, subnetMask: string): number => {
  if (!ip) {
    throw new Error("IP address is required");
  }
  if (!subnetMask) {
    throw new Error("Subnet mask is required");
  }

  // Validate IP format
  const ipParts = ip.split(".");
  if (
    ipParts.length !== 4 ||
    !ipParts.every((part) => {
      const number_ = Number.parseInt(part, 10);
      return !Number.isNaN(number_) && number_ >= 0 && number_ <= 255;
    })
  ) {
    throw new TypeError("Invalid IP address or subnet mask");
  }

  // Validate subnet mask format
  const maskParts = subnetMask.split(".");
  if (
    maskParts.length !== 4 ||
    !maskParts.every((part) => {
      const number_ = Number.parseInt(part, 10);
      return !Number.isNaN(number_) && number_ >= 0 && number_ <= 255;
    })
  ) {
    throw new TypeError("Invalid IP address or subnet mask");
  }

  try {
    const networkAddress =
      ipToLong(ip) & cidrToLong(subnetMaskToCidr(subnetMask));
    return networkAddress >>> 0; // Convert to unsigned 32-bit integer
  } catch {
    throw new TypeError("Invalid IP address or subnet mask");
  }
};
