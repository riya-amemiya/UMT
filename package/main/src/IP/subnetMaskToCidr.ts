/**
 * Converts a subnet mask to CIDR notation
 * @param {string} subnetMask - IPv4 subnet mask (e.g., "255.255.255.0")
 * @returns {number} CIDR notation (0-32)
 */
export const subnetMaskToCidr = (subnetMask: string): number => {
  // Parse octets and validate format
  const octets = subnetMask.split(".");

  // Validate each octet
  const binaryOctets = octets.map((octet) => {
    const number_ = Number.parseInt(octet, 10);
    return number_.toString(2).padStart(8, "0");
  });

  // Join octets and count consecutive 1s
  const binaryString = binaryOctets.join("");

  // Count 1s
  const cidr = binaryString.match(/1/g)?.length ?? 0;
  return cidr;
};
