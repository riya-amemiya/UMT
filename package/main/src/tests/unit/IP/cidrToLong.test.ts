import { cidrToLong } from "@/IP/cidrToLong";

describe("cidrToLong", () => {
  describe("valid CIDR values", () => {
    test.each([
      // [CIDR, expected subnet mask value]
      [32, 0xff_ff_ff_ff], // 255.255.255.255
      [24, 0xff_ff_ff_00], // 255.255.255.0
      [16, 0xff_ff_00_00], // 255.255.0.0
      [8, 0xff_00_00_00], // 255.0.0.0
      [0, 0x00_00_00_00], // 0.0.0.0
      [1, 0x80_00_00_00], // 128.0.0.0
      [31, 0xff_ff_ff_fe], // 255.255.255.254
    ])("should convert CIDR %i to subnet mask %i", (cidr, expected) => {
      expect(cidrToLong(cidr)).toBe(expected);
    });
  });

  describe("invalid CIDR values", () => {
    test.each([
      [-1, "CIDR must be an integer between 0 and 32"],
      [33, "CIDR must be an integer between 0 and 32"],
      [3.5, "CIDR must be an integer between 0 and 32"],
      [Number.NaN, "CIDR must be an integer between 0 and 32"],
      [Number.POSITIVE_INFINITY, "CIDR must be an integer between 0 and 32"],
    ])("should throw error for invalid CIDR %p", (cidr, expectedError) => {
      expect(() => cidrToLong(cidr)).toThrow(expectedError);
    });
  });
});
