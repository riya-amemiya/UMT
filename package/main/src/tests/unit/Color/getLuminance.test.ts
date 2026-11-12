import { getLuminance } from "@/Color/getLuminance";

describe("getLuminance", () => {
  test.each([
    [{ r: 255, g: 255, b: 255 }, 1],
    [{ r: 0, g: 0, b: 0 }, 0],
    [{ r: 255, g: 0, b: 0 }, 0.2126],
    [{ r: 0, g: 255, b: 0 }, 0.7152],
    [{ r: 0, g: 0, b: 255 }, 0.0722],
    [{ r: 128, g: 128, b: 128 }, 0.215_860_500_113_899_23],
    [{ r: 119, g: 119, b: 119 }, 0.184_474_994_500_441],
  ])("returns the relative luminance of %p", (rgba, expected) => {
    expect(getLuminance(rgba)).toBeCloseTo(expected, 12);
  });

  test("uses the linear segment at or below 0.03928", () => {
    expect(getLuminance({ r: 10, g: 10, b: 10 })).toBeCloseTo(
      10 / 255 / 12.92,
      15,
    );
    expect(getLuminance({ r: 1, g: 2, b: 3 })).toBeCloseTo(
      (0.2126 * 1 + 0.7152 * 2 + 0.0722 * 3) / 255 / 12.92,
      15,
    );
  });

  test("ignores alpha", () => {
    expect(getLuminance({ r: 128, g: 64, b: 32, a: 0 })).toBe(
      getLuminance({ r: 128, g: 64, b: 32 }),
    );
  });
});
