import { subnetMaskToCidr } from "@/IP/subnetMaskToCidr";

describe("subnetMaskToCidr", () => {
  test("正しいCIDR値を返す", () => {
    expect(subnetMaskToCidr("255.255.255.0")).toBe(24);
    expect(subnetMaskToCidr("255.255.0.0")).toBe(16);
    expect(subnetMaskToCidr("255.0.0.0")).toBe(8);
  });

  test("無効なサブネットマスクの場合は0を返す", () => {
    expect(subnetMaskToCidr("255.255.255.256")).toBe(0);
    expect(subnetMaskToCidr("")).toBe(0);
  });
});
