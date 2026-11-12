import { cidrToLong } from "./cidrToLong";
import { ipToLong } from "./ipToLong";
import { longToIp } from "./longToIp";

/**
Parses CIDR notation into its network address and prefix length
@param {string} cidr - CIDR notation (e.g., "192.168.0.0/24")
@returns {{ networkAddress: string; prefixLength: number }} Network address with the host bits cleared and the prefix length (0-32)
@example parseCidr("192.168.0.0/24"); // { networkAddress: "192.168.0.0", prefixLength: 24 }
@example parseCidr("192.168.1.130/25"); // { networkAddress: "192.168.1.128", prefixLength: 25 }
*/
export const parseCidr = (
  cidr: string,
): { networkAddress: string; prefixLength: number } => {
  const separatorIndex = cidr.indexOf("/");
  const prefixLength = Number.parseInt(cidr.slice(separatorIndex + 1), 10);
  return {
    networkAddress: longToIp(
      ipToLong(cidr.slice(0, separatorIndex)) & cidrToLong(prefixLength),
    ),
    prefixLength,
  };
};
