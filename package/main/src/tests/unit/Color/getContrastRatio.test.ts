import { getContrastRatio } from "@/Color/getContrastRatio";

describe("getContrastRatio", () => {
  const black = { r: 0, g: 0, b: 0 };
  const white = { r: 255, g: 255, b: 255, a: 1 };

  test("returns 21 for black and white", () => {
    expect(getContrastRatio(black, white)).toBeCloseTo(21, 12);
  });

  test("does not depend on argument order", () => {
    const gray = { r: 119, g: 119, b: 119 };
    expect(getContrastRatio(white, gray)).toBeCloseTo(
      4.478_089_453_577_214,
      12,
    );
    expect(getContrastRatio(gray, white)).toBe(getContrastRatio(white, gray));
  });

  test("returns 1 for identical colors", () => {
    expect(getContrastRatio(white, white)).toBe(1);
    expect(getContrastRatio(black, black)).toBe(1);
  });

  test("compares two saturated colors", () => {
    expect(
      getContrastRatio({ r: 255, g: 0, b: 0 }, { r: 0, g: 0, b: 255 }),
    ).toBeCloseTo(2.148_936_170_212_766, 12);
  });
});
