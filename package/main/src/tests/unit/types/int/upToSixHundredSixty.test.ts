import type { UpToSixHundredSixty } from "$/int/upToSixHundredSixty";

describe("UpToSixHundredSixty", () => {
  it("UpToSixHundredSixty", () => {
    const _: UpToSixHundredSixty[] = [
      650, 651, 652, 653, 654, 655, 656, 657, 658, 659, 660,
    ];
    expect(_).toEqual(_);
  });
  it("649 extends UpToSixHundredSixty", () => {
    const _: 649 extends UpToSixHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("661 extends UpToSixHundredSixty", () => {
    const _: 661 extends UpToSixHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
});
