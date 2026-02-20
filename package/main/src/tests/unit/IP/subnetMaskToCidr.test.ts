import { subnetMaskToCidr } from "@/IP/subnetMaskToCidr";

describe("subnetMaskToCidr", () => {
  describe("valid subnet masks", () => {
    test.each([
      // [subnet mask, expected CIDR]
      ["255.255.255.255", 32], // All bits set
      ["255.255.255.0", 24], // Common /24 network
      ["255.255.0.0", 16], // Common /16 network
      ["255.0.0.0", 8], // Common /8 network
      ["255.255.254.0", 23], // Less common /23 network
      ["255.255.255.252", 30], // Small /30 network
      ["255.255.255.248", 29], // Small /29 network
      ["255.255.240.0", 20], // Medium /20 network
      ["0.0.0.0", 0], // No bits set
    ])("should convert %s to /%i", (mask, expected) => {
      expect(subnetMaskToCidr(mask)).toBe(expected);
    });
  });
});
