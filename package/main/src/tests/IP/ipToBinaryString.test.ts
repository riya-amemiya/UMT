import { ipToBinaryString } from "@/IP/ipToBinaryString";

describe("ipToBinaryString", () => {
  it("正常なIPアドレスを正しくバイナリ文字列に変換する", () => {
    expect(ipToBinaryString("192.168.0.1")).toBe(
      "11000000101010000000000000000001",
    );
  });

  it("全てのオクテットが0のIPアドレスを正しく変換する", () => {
    expect(ipToBinaryString("0.0.0.0")).toBe(
      "00000000000000000000000000000000",
    );
  });

  it("全てのオクテットが255のIPアドレスを正しく変換する", () => {
    expect(ipToBinaryString("255.255.255.255")).toBe(
      "11111111111111111111111111111111",
    );
  });

  it("エッジケース（オクテットが1桁）を正しく扱う", () => {
    expect(ipToBinaryString("1.2.3.4")).toBe(
      "00000001000000100000001100000100",
    );
  });
});
