import { cidrToLong } from "@/IP/cidrToLong";

describe("cidrToLong for IPv4", () => {
  test.each([
    // [CIDR, 期待されるサブネットマスクの数値]
    [32, 0xffffffff],
    [24, 0xffffff00],
    [16, 0xffff0000],
    [8, 0xff000000],
    [0, 0x00000000],
  ])("cidrToLong(%i) should return %i for IPv4", (cidr, expected) => {
    expect(cidrToLong(cidr)).toBe(expected);
  });
});
