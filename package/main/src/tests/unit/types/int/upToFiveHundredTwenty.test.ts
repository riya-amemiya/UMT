import type { UpToFiveHundredTwenty } from "$/int/upToFiveHundredTwenty";

describe("UpToFiveHundredTwenty", () => {
  it("UpToFiveHundredTwenty", () => {
    const _: UpToFiveHundredTwenty[] = [
      510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 520,
    ];
    expect(_).toEqual(_);
  });
  it("509 extends UpToFiveHundredTwenty", () => {
    const _: 509 extends UpToFiveHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("521 extends UpToFiveHundredTwenty", () => {
    const _: 521 extends UpToFiveHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
});
