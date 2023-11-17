import { ipToLong } from "@/IP/ipToLong";

describe("ipToLong", () => {
  it("正常なIPアドレスが正しい数値に変換される", () => {
    expect(ipToLong("128.0.0.1")).toBe(2147483649);
  });

  it("全てのオクテットが255の場合、最大値に変換される", () => {
    expect(ipToLong("255.255.255.255")).toBe(4294967295);
  });

  it("全てのオクテットが0の場合、0に変換される", () => {
    expect(ipToLong("0.0.0.0")).toBe(0);
  });

  it("空の文字列が渡された場合、0に変換される", () => {
    expect(ipToLong("")).toBe(0);
  });

  it("範囲外のIPアドレスの場合、0に変換される", () => {
    expect(ipToLong("256.0.0.0")).toBe(0);
  });
});
