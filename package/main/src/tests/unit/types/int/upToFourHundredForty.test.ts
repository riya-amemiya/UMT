import type { UpToFourHundredForty } from "$/int/upToFourHundredForty";

describe("UpToFourHundredForty", () => {
  it("UpToFourHundredForty", () => {
    const _: UpToFourHundredForty[] = [
      430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440,
    ];
    expect(_).toEqual(_);
  });
  it("429 extends UpToFourHundredForty", () => {
    const _: 429 extends UpToFourHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("441 extends UpToFourHundredForty", () => {
    const _: 441 extends UpToFourHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
});
