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

  describe("invalid IP addresses", () => {
    test.each([
      [undefined, "IP address is required"],
      ["", "IP address is required"],
      ["192.168", "Invalid IP address format"],
      ["256.1.2.3", "Invalid IP address format"],
      ["a.b.c.d", "Invalid IP address format"],
      ["-1.0.0.0", "Invalid IP address format"],
      ["192.168.1.1.1", "Invalid IP address format"],
      ["192.168..1", "Invalid IP address format"],
    ])("should throw error for %s: %s", (ip, expectedError) => {
      expect(() => ipToLong(ip as string)).toThrow(expectedError);
    });
  });
});
