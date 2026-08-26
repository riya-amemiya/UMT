/**
 * Converts a subnet mask to CIDR notation
 * @param {string} subnetMask - IPv4 subnet mask (e.g., "255.255.255.0")
 * @returns {number} CIDR notation (0-32)
 */
export const subnetMaskToCidr = (subnetMask: string): number => {
  // Count set bits per octet (n &= n - 1) instead of allocating a 32-char
  // binary string and scanning it with a regex. `>>> 0` maps NaN to 0 so
  // the loop always terminates.
  let cidr = 0;
  for (const part of subnetMask.split(".")) {
    let octet = Number.parseInt(part, 10) >>> 0;
    while (octet) {
      octet &= octet - 1;
      cidr++;
    }
  }
  return cidr;
};
