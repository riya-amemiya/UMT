import { getNetworkAddress } from "@/IP/getNetworkAddress";

describe("getNetworkAddress", () => {
  describe("valid IP addresses and subnet masks", () => {
    test.each([
      // [IP address, subnet mask, expected network address]
      ["192.168.1.1", "255.255.255.0", 0xc0a80100], // Common Class C network
      ["172.16.5.1", "255.255.0.0", 0xac100000], // Common Class B network
      ["10.0.0.15", "255.0.0.0", 0x0a000000], // Common Class A network
      ["192.168.1.1", "255.255.254.0", 0xc0a80000], // Non-standard subnet mask
      ["255.255.255.255", "255.255.255.0", 0xffffff00], // All host bits set
      ["0.0.0.0", "255.255.255.0", 0x00000000], // All bits zero
      ["192.168.1.1", "255.255.255.252", 0xc0a80100], // Small subnet (/30)
      ["10.10.10.10", "255.255.255.240", 0x0a0a0a00], // Subnet mask with /28
    ])(
      "should calculate network address for %s with mask %s",
      (ip, mask, expected) => {
        expect(getNetworkAddress(ip, mask)).toBe(expected);
      },
    );
  });

  describe("invalid inputs", () => {
    test.each([
      [undefined, "255.255.255.0", "IP address is required"],
      ["192.168.1.1", undefined, "Subnet mask is required"],
      ["", "255.255.255.0", "IP address is required"],
      ["192.168.1.1", "", "Subnet mask is required"],
      ["invalid", "255.255.255.0", "Invalid IP address or subnet mask"],
      ["192.168.1.1", "invalid", "Invalid IP address or subnet mask"],
      ["256.256.256.256", "255.255.255.0", "Invalid IP address or subnet mask"],
      ["192.168.1.1", "256.256.256.256", "Invalid IP address or subnet mask"],
    ])("should throw error for %s and %s: %s", (ip, mask, expectedError) => {
      expect(() => getNetworkAddress(ip as string, mask as string)).toThrow(
        expectedError,
      );
    });
  });

  test("should handle internal function exceptions", () => {
    // Test for exception handling in catch block (line 49)
    expect(() => getNetworkAddress("192.168.1.1", "255.255.128.3")).toThrow(
      "Invalid IP address or subnet mask",
    );
  });
});
