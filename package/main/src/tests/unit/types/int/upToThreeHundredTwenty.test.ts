import type { UpToThreeHundredTwenty } from "$/int/upToThreeHundredTwenty";

describe("UpToThreeHundredTwenty", () => {
  it("UpToThreeHundredTwenty", () => {
    const _: UpToThreeHundredTwenty[] = [
      310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320,
    ];
    expect(_).toEqual(_);
  });
  it("309 extends UpToThreeHundredTwenty", () => {
    const _: 309 extends UpToThreeHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("321 extends UpToThreeHundredTwenty", () => {
    const _: 321 extends UpToThreeHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
});
