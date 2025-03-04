import { hslaToRgba } from "@/Color/hslaToRgba";

describe("hslaToRgba function", () => {
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
  ])(
    "should convert HSLA(%i, %i%, %i%, %i) to RGBA",
    (h, s, l, a, expected) => {
      expect(hslaToRgba(h, s, l, a)).toEqual(expected);
    },
  );

  it.each([
    [0, 0, 0, { r: 0, g: 0, b: 0, a: 1 }],
    [360, 100, 100, { r: 255, g: 255, b: 255, a: 1 }],
    [0, 100, 50, { r: 255, g: 0, b: 0, a: 1 }],
  ])("should convert HSLA(%i, %i%, %i%) to RGBA", (h, s, l, expected) => {
    expect(hslaToRgba(h, s, l)).toEqual(expected);
  });

  it.each([
    [240, 100, 25, 1, { r: 0, g: 0, b: 127.5, a: 1 }],
    [120, 100, 25, 1, { r: 0, g: 127.5, b: 0, a: 1 }],
    [0, 100, 25, 1, { r: 127.5, g: 0, b: 0, a: 1 }],
    [60, 100, 25, 1, { r: 127.5, g: 127.5, b: 0, a: 1 }],
    [180, 100, 25, 1, { r: 0, g: 127.5, b: 127.5, a: 1 }],
    [300, 100, 25, 1, { r: 127.5, g: 0, b: 127.5, a: 1 }],
    [0, 0, 25, 1, { r: 63.75, g: 63.75, b: 63.75, a: 1 }],
  ])(
    "should convert HSLA(%i, %i%, %i%, %i) to RGBA",
    (h, s, l, a, expected) => {
      expect(hslaToRgba(h, s, l, a)).toEqual(expected);
    },
  );

  it("should handle invalid values", () => {
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
