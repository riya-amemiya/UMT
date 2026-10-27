import type { UpToFourHundredSixty } from "$/int/upToFourHundredSixty";

describe("UpToFourHundredSixty", () => {
  it("UpToFourHundredSixty", () => {
    const _: UpToFourHundredSixty[] = [
      450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460,
    ];
    expect(_).toEqual(_);
  });
  it("449 extends UpToFourHundredSixty", () => {
    const _: 449 extends UpToFourHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("461 extends UpToFourHundredSixty", () => {
    const _: 461 extends UpToFourHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
});
