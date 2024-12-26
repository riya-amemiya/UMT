// cidrToSubnetMask.test.ts

import { cidrToSubnetMask } from "@/IP/cidrToSubnetMask";

describe("cidrToSubnetMask", () => {
  // IPv4のテスト
  test("IPv4 CIDR /24 をサブネットマスクに変換", () => {
    expect(cidrToSubnetMask(24)).toBe("255.255.255.0");
  });

  test("IPv4 CIDR /16 をサブネットマスクに変換", () => {
    expect(cidrToSubnetMask(16)).toBe("255.255.0.0");
  });

  test("IPv4 CIDR /8 をサブネットマスクに変換", () => {
    expect(cidrToSubnetMask(8)).toBe("255.0.0.0");
  });
});
