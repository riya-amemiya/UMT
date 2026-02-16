import { getNetworkAddress } from "@/IP/getNetworkAddress";

describe("getNetworkAddress", () => {
  describe("valid IP addresses and subnet masks", () => {
    test.each([
      // [IP address, subnet mask, expected network address]
      ["192.168.1.1", "255.255.255.0", 0xc0_a8_01_00], // Common Class C network
      ["172.16.5.1", "255.255.0.0", 0xac_10_00_00], // Common Class B network
      ["10.0.0.15", "255.0.0.0", 0x0a_00_00_00], // Common Class A network
      ["192.168.1.1", "255.255.254.0", 0xc0_a8_00_00], // Non-standard subnet mask
      ["255.255.255.255", "255.255.255.0", 0xff_ff_ff_00], // All host bits set
      ["0.0.0.0", "255.255.255.0", 0x00_00_00_00], // All bits zero
      ["192.168.1.1", "255.255.255.252", 0xc0_a8_01_00], // Small subnet (/30)
      ["10.10.10.10", "255.255.255.240", 0x0a_0a_0a_00], // Subnet mask with /28
    ])("should calculate network address for %s with mask %s", (ip, mask, expected) => {
      expect(getNetworkAddress(ip, mask)).toBe(expected);
    });
  });
});
