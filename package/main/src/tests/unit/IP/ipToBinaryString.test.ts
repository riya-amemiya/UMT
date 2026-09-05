import { ipToBinaryString } from "@/IP/ipToBinaryString";
import { ipToLong } from "@/IP/ipToLong";
import { longToIp } from "@/IP/longToIp";

describe("ipToBinaryString", () => {
  describe("valid IP addresses", () => {
    test.each([
      // [IP address, expected binary string]
      ["192.168.1.1", "11000000101010000000000100000001"],
      ["192.168.0.1", "11000000101010000000000000000001"], // Common private network
      ["0.0.0.0", "00000000000000000000000000000000"], // All zeros
      ["255.255.255.255", "11111111111111111111111111111111"], // All ones
      ["1.2.3.4", "00000001000000100000001100000100"], // Single digit octets
      ["10.0.0.1", "00001010000000000000000000000001"], // Class A private
      ["172.16.0.1", "10101100000100000000000000000001"], // Class B private
      ["127.0.0.1", "01111111000000000000000000000001"], // Localhost
      ["169.254.0.1", "10101001111111100000000000000001"], // Link-local
    ])("should convert %s to binary", (ip, expected) => {
      expect(ipToBinaryString(ip)).toBe(expected);
    });
  });

  describe("edge cases", () => {
    test.each([
      // [IP address, expected binary string]
      ["0.0.0.0", "00000000000000000000000000000000"],
      ["1.1.1.1", "00000001000000010000000100000001"],
      ["255.255.255.255", "11111111111111111111111111111111"],
      ["128.0.0.0", "10000000000000000000000000000000"],
      ["0.255.0.255", "00000000111111110000000011111111"],
    ])("should handle boundary values: %s", (ip, expected) => {
      expect(ipToBinaryString(ip)).toBe(expected);
    });
  });

  it("should match ipToLong bits for listed addresses", () => {
    const ips = [
      "192.168.0.1",
      "0.0.0.0",
      "255.255.255.255",
      "1.2.3.4",
      "10.0.0.1",
      "172.16.0.1",
      "127.0.0.1",
      "169.254.0.1",
      "8.8.8.8",
      "128.0.0.0",
      "0.255.0.255",
      "192.168.1.1",
    ];
    for (const ip of ips) {
      const binary = ipToBinaryString(ip);
      expect(binary).toBe(ipToLong(ip).toString(2).padStart(32, "0"));
      expect(binary).toHaveLength(32);
    }
  });

  it("should round-trip through longToIp", () => {
    const longs = [
      0, 1, 0x7f_00_00_01, 0xc0_a8_00_01, 0xff_ff_ff_ff, 0x01_02_03_04,
      0x0a_00_00_01, 0xac_10_00_01, 0x80_00_00_00, 0x00_ff_00_ff,
    ];
    for (const longValue of longs) {
      const ip = longToIp(longValue);
      expect(ipToBinaryString(ip)).toBe(
        longValue.toString(2).padStart(32, "0"),
      );
      expect(ipToLong(ip)).toBe(longValue);
    }
  });
});
