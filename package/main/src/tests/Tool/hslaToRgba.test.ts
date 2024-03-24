import { hslaToRgba } from "@/Tool/hslaToRgba";

describe("hslaToRgba関数のテスト", () => {
  it.each([
    [0, 100, 50, 1, { r: 255, g: 0, b: 0, a: 1 }],
    [120, 100, 50, 1, { r: 0, g: 255, b: 0, a: 1 }],
    [240, 100, 50, 1, { r: 0, g: 0, b: 255, a: 1 }],
    [60, 100, 50, 1, { r: 255, g: 255, b: 0, a: 1 }],
    [180, 100, 50, 1, { r: 0, g: 255, b: 255, a: 1 }],
    [300, 100, 50, 1, { r: 255, g: 0, b: 255, a: 1 }],
    [0, 0, 100, 1, { r: 255, g: 255, b: 255, a: 1 }],
    [0, 0, 0, 1, { r: 0, g: 0, b: 0, a: 1 }],
    [0, 100, 50, 0.5, { r: 255, g: 0, b: 0, a: 0.5 }],
  ])("HSLA(%i, %i%, %i%, %i)をRGBAに変換する", (h, s, l, a, expected) => {
    expect(hslaToRgba(h, s, l, a)).toEqual(expected);
  });

  it("不正な値を渡した場合の挙動", () => {
    expect(hslaToRgba(-60, 100, 50, 1)).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(hslaToRgba(420, 100, 50, 1)).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(hslaToRgba(0, -50, 50, 1)).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(hslaToRgba(0, 150, 50, 1)).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(hslaToRgba(0, 100, -20, 1)).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(hslaToRgba(0, 100, 120, 1)).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(hslaToRgba(0, 100, 50, -0.5)).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(hslaToRgba(0, 100, 50, 1.5)).toEqual({ r: 0, g: 0, b: 0, a: 1 });
  });
});
