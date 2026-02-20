import { ipToLong } from "@/IP/ipToLong";

describe("ipToLong", () => {
  describe("valid IP addresses", () => {
    test.each([
      // [IP address, expected long value]
      ["192.168.0.1", 0xc0_a8_00_01], // Common private network
      ["128.0.0.1", 0x80_00_00_01], // Class B start
      ["10.0.0.1", 0x0a_00_00_01], // Class A private
      ["172.16.0.1", 0xac_10_00_01], // Class B private
      ["255.255.255.255", 0xff_ff_ff_ff], // Maximum value
      ["0.0.0.0", 0x00_00_00_00], // Minimum value
      ["127.0.0.1", 0x7f_00_00_01], // Localhost
      ["1.2.3.4", 0x01_02_03_04], // Simple incremental
    ])("should convert %s to %i", (ip, expected) => {
      expect(ipToLong(ip)).toBe(expected);
    });
  });
});
