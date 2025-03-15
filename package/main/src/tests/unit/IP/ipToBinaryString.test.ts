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

  describe("invalid IP addresses", () => {
    test.each([
      [undefined, "IP address is required"],
      ["", "IP address is required"],
      ["192.168", "Invalid IP address format"],
      ["a.b.c.d", "Invalid IP address format"],
      ["256.1.2.3", "Invalid IP address format"],
      ["-1.1.1.1", "Invalid IP address format"],
      ["1.2.3.4.5", "Invalid IP address format"],
      ["192.168.1", "Invalid IP address format"],
      ["192.168.1.1.1", "Invalid IP address format"],
      ["192.168.1.", "Invalid IP address format"],
      ["192.168..1", "Invalid IP address format"],
      [".192.168.1", "Invalid IP address format"],
      ["192,168,1,1", "Invalid IP address format"],
      ["192.168.1.1.", "Invalid IP address format"],
      ["192.168.1.+1", "Invalid IP address format"],
      ["256.256.256.256", "Invalid IP address format"],
      ["999.999.999.999", "Invalid IP address format"],
    ])("should throw error for %s: %s", (ip, expectedError) => {
      expect(() => ipToBinaryString(ip as string)).toThrow(expectedError);
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

    test.each([
      "0000",
      "00.00",
      "0.0.0",
      "192.168.01.1",
      "192.168.1.01",
      "010.020.030.040",
    ])("should reject IP with invalid format: %s", (ip) => {
      expect(() => ipToBinaryString(ip)).toThrow("Invalid IP address format");
    });
  });
});
