import type { UpToFiveHundredSeventy } from "$/int/upToFiveHundredSeventy";

describe("UpToFiveHundredSeventy", () => {
  it("UpToFiveHundredSeventy", () => {
    const _: UpToFiveHundredSeventy[] = [
      560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570,
    ];
    expect(_).toEqual(_);
  });
  it("559 extends UpToFiveHundredSeventy", () => {
    const _: 559 extends UpToFiveHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
  it("571 extends UpToFiveHundredSeventy", () => {
    const _: 571 extends UpToFiveHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
});
