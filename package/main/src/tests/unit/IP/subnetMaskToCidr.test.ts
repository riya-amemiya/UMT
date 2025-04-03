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

  describe("invalid subnet masks", () => {
    test.each([
      [undefined, "Subnet mask is required"],
      ["", "Subnet mask is required"],
      ["192.168", "Invalid subnet mask format"],
      ["256.255.255.0", "Invalid subnet mask format"],
      ["255.255.255.256", "Invalid subnet mask format"],
      ["255.-1.255.0", "Invalid subnet mask format"],
      ["255.255.255.abc", "Invalid subnet mask format"],
      ["a.b.c.d", "Invalid subnet mask format"],
      [
        "255.255.255.1",
        "Invalid subnet mask: must be consecutive 1s followed by 0s",
      ], // Non-continuous mask
      [
        "255.0.255.0",
        "Invalid subnet mask: must be consecutive 1s followed by 0s",
      ], // Non-continuous mask
      [
        "254.255.255.0",
        "Invalid subnet mask: must be consecutive 1s followed by 0s",
      ], // Invalid bit pattern
    ])("should throw error for %s: %s", (mask, expectedError) => {
      expect(() => subnetMaskToCidr(mask as string)).toThrow(expectedError);
    });
  });
});
