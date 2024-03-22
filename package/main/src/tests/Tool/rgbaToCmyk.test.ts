import { rgbaToCmyk } from "@/Tool/rgbaToCmyk";

describe("rgbaToCmyk", () => {
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
  ])("RGBA(%o)をCMYKに変換する", (input, expected) => {
    expect(rgbaToCmyk(input)).toEqual(expected);
  });
});
