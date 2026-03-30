import { ipToBinaryString } from "@/IP/ipToBinaryString";

describe("ipToBinaryString", () => {
  describe("valid IP addresses", () => {
    test.each([
      // [IP address, expected binary string]
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

  describe("invalid inputs should throw", () => {
    test.each([
      ["", "non-empty string"],
      ["1.2.3", "4 octets"],
      ["1.2.3.4.5", "4 octets"],
      ["256.0.0.1", "out of range"],
      ["-1.0.0.0", 'invalid octet "-1"'],
      ["abc.0.0.1", 'invalid octet "abc"'],
      ["1.2.3.999", "out of range"],
      ["1.2.3.", 'invalid octet ""'],
      ["1.2.3.0x1", 'invalid octet "0x1"'],
      ["01onal.0.0.1", 'invalid octet "01onal"'],
    ])("should throw for invalid input: %s", (ip, expectedMsg) => {
      expect(() => ipToBinaryString(ip)).toThrow(expectedMsg);
    });
  });
});
