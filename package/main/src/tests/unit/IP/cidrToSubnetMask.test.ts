import { cidrToSubnetMask } from "@/IP/cidrToSubnetMask";

describe("cidrToSubnetMask", () => {
  describe("valid CIDR values", () => {
    test.each([
      // [CIDR, expected subnet mask]
      [32, "255.255.255.255"], // Full mask
      [24, "255.255.255.0"], // Common class C
      [16, "255.255.0.0"], // Common class B
      [8, "255.0.0.0"], // Common class A
      [0, "0.0.0.0"], // No mask
      [1, "128.0.0.0"], // First bit
      [31, "255.255.255.254"], // Second to last
      [28, "255.255.255.240"], // Common subnet
      [20, "255.255.240.0"], // Less common subnet
    ])("should convert CIDR /%i to subnet mask %s", (cidr, expected) => {
      expect(cidrToSubnetMask(cidr)).toBe(expected);
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
      expect(() => cidrToSubnetMask(cidr)).toThrow(expectedError);
    });
  });
});
