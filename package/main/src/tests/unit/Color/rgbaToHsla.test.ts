import { rgbaToHsla } from "@/Color/rgbaToHsla";

describe("rgbaToHsla", () => {
  it("should throw error for invalid rgba values", () => {
    expect(() => rgbaToHsla({ r: -1, g: 0, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHsla({ r: 256, g: 0, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHsla({ r: 0, g: -1, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHsla({ r: 0, g: 256, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHsla({ r: 0, g: 0, b: -1, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHsla({ r: 0, g: 0, b: 256, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHsla({ r: 0, g: 0, b: 0, a: -0.1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHsla({ r: 0, g: 0, b: 0, a: 1.1 })).toThrow(
      "Invalid rgba value",
    );
  });

  it("should handle edge cases correctly", () => {
    // Black
    expect(rgbaToHsla({ r: 0, g: 0, b: 0 })).toEqual({
      h: 0,
      s: 0,
      l: 0,
      a: 1,
    });
    // White
    expect(rgbaToHsla({ r: 255, g: 255, b: 255 })).toEqual({
      h: 0,
      s: 0,
      l: 100,
      a: 1,
    });
  });

  it.each([
    [
      { r: 100, g: 100, b: 100 },
      { h: 0, s: 0, l: 39.22, a: 1 },
    ],
    [
      { r: 255, g: 0, b: 0 },
      { h: 0, s: 100, l: 50, a: 1 },
    ],
    [
      { r: 0, g: 255, b: 0 },
      { h: 120, s: 100, l: 50, a: 1 },
    ],
    [
      { r: 0, g: 0, b: 255 },
      { h: 240, s: 100, l: 50, a: 1 },
    ],
    [
      { r: 255, g: 0, b: 0, a: 0.5 },
      { h: 0, s: 100, l: 50, a: 0.5 },
    ],
    [
      { r: 255, g: 255, b: 0 },
      { h: 60, s: 100, l: 50, a: 1 },
    ],
    [
      { r: 0, g: 0, b: 255, a: 0.7 },
      { h: 240, s: 100, l: 50, a: 0.7 },
    ],
    [
      { r: 173, g: 216, b: 230 },
      { h: 194.74, s: 53.27, l: 79.02, a: 1 },
    ],
    [
      { r: 255, g: 0, b: 100 },
      { h: 336.47, s: 100, l: 50, a: 1 },
    ],
    [
      { r: 100, g: 0, b: 255 },
      { h: 263.53, s: 100, l: 50, a: 1 },
    ],
  ])("should convert RGB(%o) to HSLA", (input, expected) => {
    expect(rgbaToHsla(input)).toEqual(expected);
  });
});
