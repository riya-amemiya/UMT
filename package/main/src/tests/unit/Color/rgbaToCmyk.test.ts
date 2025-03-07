import { rgbaToCmyk } from "@/Color/rgbaToCmyk";

describe("rgbaToCmyk", () => {
  it("should throw an error for invalid rgba values", () => {
    expect(() => rgbaToCmyk({ r: -1, g: 0, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToCmyk({ r: 256, g: 0, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToCmyk({ r: 0, g: -1, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToCmyk({ r: 0, g: 256, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToCmyk({ r: 0, g: 0, b: -1, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToCmyk({ r: 0, g: 0, b: 256, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToCmyk({ r: 0, g: 0, b: 0, a: -0.1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToCmyk({ r: 0, g: 0, b: 0, a: 1.1 })).toThrow(
      "Invalid rgba value",
    );
  });

  it.each([
    [
      { r: 255, g: 255, b: 255, a: 1 },
      { c: 0, m: 0, y: 0, k: 0, a: 1 },
    ],
    [
      { r: 0, g: 0, b: 0, a: 1 },
      { c: 0, m: 0, y: 0, k: 100, a: 1 },
    ],
    [
      { r: 255, g: 0, b: 0, a: 0.5 },
      { c: 0, m: 100, y: 100, k: 0, a: 0.5 },
    ],
    [
      { r: 0, g: 255, b: 0, a: 0.7 },
      { c: 100, m: 0, y: 100, k: 0, a: 0.7 },
    ],
    [
      { r: 0, g: 0, b: 255, a: 0.3 },
      { c: 100, m: 100, y: 0, k: 0, a: 0.3 },
    ],
    [
      { r: 255, g: 255, b: 255 },
      { c: 0, m: 0, y: 0, k: 0, a: 1 },
    ],
  ])("should convert RGBA(%o) to CMYK", (input, expected) => {
    expect(rgbaToCmyk(input)).toEqual(expected);
  });
});
