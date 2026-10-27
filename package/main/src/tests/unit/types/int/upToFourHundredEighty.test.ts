import type { UpToFourHundredEighty } from "$/int/upToFourHundredEighty";

describe("UpToFourHundredEighty", () => {
  it("UpToFourHundredEighty", () => {
    const _: UpToFourHundredEighty[] = [
      470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 480,
    ];
    expect(_).toEqual(_);
  });
  it("469 extends UpToFourHundredEighty", () => {
    const _: 469 extends UpToFourHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("481 extends UpToFourHundredEighty", () => {
    const _: 481 extends UpToFourHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
});
