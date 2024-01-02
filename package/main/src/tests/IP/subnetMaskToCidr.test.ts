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
    expect(subnetMaskToCidr("255.-1.255.0")).toBe(0);
    expect(subnetMaskToCidr("255.255.255.abc")).toBe(0);
  });

  test("すべてのオクテットが有効な数値の場合", () => {
    expect(subnetMaskToCidr("255.255.255.255")).toBe(32);
  });

  test("1つ以上のオクテットが0の場合", () => {
    expect(subnetMaskToCidr("255.0.0.0")).toBe(8);
    expect(subnetMaskToCidr("255.255.0.0")).toBe(16);
  });

  test("1つ以上のオクテットが255でない場合", () => {
    expect(subnetMaskToCidr("255.255.254.0")).toBe(23);
    expect(subnetMaskToCidr("255.255.255.252")).toBe(30);
  });
});
