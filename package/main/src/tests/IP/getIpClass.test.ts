import { getIpClass } from "@/IP/getIpClass";

describe("getIpClass", () => {
  // IPv4のテストケース
  test("IPクラスAを識別する", () => {
    expect(getIpClass("1.2.3.4")).toBe("A");
  });

  test("IPクラスBを識別する", () => {
    expect(getIpClass("128.1.2.3")).toBe("B");
  });

  test("IPクラスCを識別する", () => {
    expect(getIpClass("192.0.2.1")).toBe("C");
  });

  test("IPクラスDを識別する", () => {
    expect(getIpClass("224.0.0.1")).toBe("D");
  });

  test("IPクラスEを識別する", () => {
    expect(getIpClass("240.0.0.1")).toBe("E");
  });

  test("無効な範囲のIPアドレス", () => {
    expect(getIpClass("")).toBe("");
  });
});
