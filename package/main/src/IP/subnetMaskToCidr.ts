/**
 * Converts a subnet mask to CIDR notation
 * @param {string} subnetMask - IPv4 subnet mask (e.g., "255.255.255.0")
 * @returns {number} CIDR notation (0-32)
 * @throws {Error} If subnet mask is invalid
 */
export const subnetMaskToCidr = (subnetMask: string): number => {
  if (!subnetMask) {
    throw new Error("Subnet mask is required");
  }

  // Parse octets and validate format
  const octets = subnetMask.split(".");
  if (octets.length !== 4) {
    throw new Error("Invalid subnet mask format");
  }

  // Validate each octet
  const binaryOctets = octets.map((octet) => {
    const number_ = Number.parseInt(octet, 10);
    if (Number.isNaN(number_) || number_ < 0 || number_ > 255) {
      throw new Error("Invalid subnet mask format");
    }
    return number_.toString(2).padStart(8, "0");
  });

  // Join octets and count consecutive 1s
  const binaryString = binaryOctets.join("");
  const match = /^1*0*$/.exec(binaryString);
  if (!match) {
    throw new Error(
      "Invalid subnet mask: must be consecutive 1s followed by 0s",
    );
  }

  // Count 1s
  const cidr = binaryString.match(/1/g)?.length ?? 0;
  return cidr;
};
