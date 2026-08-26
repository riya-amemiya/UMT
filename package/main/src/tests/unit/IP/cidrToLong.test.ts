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

  it("should match 2^32 - 2^(32-cidr) for every prefix length 0–32", () => {
    for (let cidr = 0; cidr <= 32; cidr++) {
      const expected = Number(2n ** 32n - 2n ** (32n - BigInt(cidr)));
      expect(cidrToLong(cidr)).toBe(expected);
    }
  });
});
