import type { UpToThreeHundredSixty } from "$/int/upToThreeHundredSixty";

describe("UpToThreeHundredSixty", () => {
  it("UpToThreeHundredSixty", () => {
    const _: UpToThreeHundredSixty[] = [
      350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360,
    ];
    expect(_).toEqual(_);
  });
  it("349 extends UpToThreeHundredSixty", () => {
    const _: 349 extends UpToThreeHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("361 extends UpToThreeHundredSixty", () => {
    const _: 361 extends UpToThreeHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
});
