import { getNetworkAddress } from "@/IP/getNetworkAddress";

describe("getNetworkAddress function", () => {
  it("正常なIPアドレスとサブネットマスクが正しいネットワークアドレスに変換される", () => {
    // 一般的なケース
    expect(getNetworkAddress("192.168.1.1", "255.255.255.0")).toBe(0xc0a80100);
    expect(getNetworkAddress("172.16.5.1", "255.255.0.0")).toBe(0xac100000);
    expect(getNetworkAddress("10.0.0.15", "255.0.0.0")).toBe(0x0a000000);

    // エッジケース（全てのホストビットが1または0）
    expect(getNetworkAddress("255.255.255.255", "255.255.255.0")).toBe(
      0xffffff00,
    );
    expect(getNetworkAddress("0.0.0.0", "255.255.255.0")).toBe(0x00000000);

    // 異なるCIDR値を持つサブネットマスク
    expect(getNetworkAddress("192.168.1.1", "255.255.254.0")).toBe(0xc0a80000);
    // expect(getNetworkAddress("172.16.5.1", "255.255.255.128")).toBe(0xac100501);
  });
});
