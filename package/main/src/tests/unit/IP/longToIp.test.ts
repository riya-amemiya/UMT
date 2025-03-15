import { longToIp } from "@/IP/longToIp";

describe("longToIp", () => {
  describe("valid 32-bit integers", () => {
    test.each([
      // [long value, expected IP]
      [0xc0a80001, "192.168.0.1"], // Common private network
      [0x80000001, "128.0.0.1"], // Class B start
      [0x0a000001, "10.0.0.1"], // Class A private
      [0xac100001, "172.16.0.1"], // Class B private
      [0xffffffff, "255.255.255.255"], // Maximum value
      [0x00000000, "0.0.0.0"], // Minimum value
      [0x7f000001, "127.0.0.1"], // Localhost
      [0x01020304, "1.2.3.4"], // Simple incremental
    ])("should convert %i to %s", (long, expected) => {
      expect(longToIp(long)).toBe(expected);
    });
  });

  describe("invalid inputs", () => {
    test.each([
      [undefined, "Input must be a valid 32-bit unsigned integer"],
      [null, "Input must be a valid 32-bit unsigned integer"],
      [Number.NaN, "Input must be a valid 32-bit unsigned integer"],
      [
        Number.POSITIVE_INFINITY,
        "Input must be a valid 32-bit unsigned integer",
      ],
      [-1, "Input must be a valid 32-bit unsigned integer"],
      [0x100000000, "Input must be a valid 32-bit unsigned integer"], // Too large
      [1.5, "Input must be a valid 32-bit unsigned integer"], // Not an integer
      ["123", "Input must be a valid 32-bit unsigned integer"], // Wrong type
    ])("should throw error for %p: %s", (input, expectedError) => {
      expect(() => longToIp(input as number)).toThrow(expectedError);
    });
  });
});
