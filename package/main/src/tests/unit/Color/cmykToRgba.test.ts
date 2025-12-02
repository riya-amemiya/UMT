import { cmykToRgba } from "@/Color/cmykToRgba";

describe("cmykToRgba function", () => {
  it.each([
    [100, 100, 0, 60.78, 1, { r: 0, g: 0, b: 100, a: 1 }],
    [0, 0, 0, 0, 1, { r: 255, g: 255, b: 255, a: 1 }],
    [0, 100, 100, 0, 0.5, { r: 255, g: 0, b: 0, a: 0.5 }],
    [100, 0, 100, 0, 0.7, { r: 0, g: 255, b: 0, a: 0.7 }],
    [100, 100, 0, 0, 0.3, { r: 0, g: 0, b: 255, a: 0.3 }],
    [50, 50, 50, 50, 1, { r: 64, g: 64, b: 64, a: 1 }],
  ])("should convert CMYK(%i, %i, %i, %i, %i) to RGBA", (c, m, y, k, a, expected) => {
    expect(cmykToRgba(c, m, y, k, a)).toEqual(expected);
  });

  it("should handle invalid values", () => {
    expect(cmykToRgba(-1, 100, 100, 0)).toEqual({ r: 255, g: 0, b: 0, a: 1 });
    expect(cmykToRgba(100, 100, 100, 100)).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(cmykToRgba(100, 100, 100, 100, -1)).toEqual({
      r: 0,
      g: 0,
      b: 0,
      a: 0,
    });
  });
});
