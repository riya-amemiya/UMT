import type { UpToSixHundredForty } from "$/int/upToSixHundredForty";

describe("UpToSixHundredForty", () => {
  it("UpToSixHundredForty", () => {
    const _: UpToSixHundredForty[] = [
      630, 631, 632, 633, 634, 635, 636, 637, 638, 639, 640,
    ];
    expect(_).toEqual(_);
  });
  it("629 extends UpToSixHundredForty", () => {
    const _: 629 extends UpToSixHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("641 extends UpToSixHundredForty", () => {
    const _: 641 extends UpToSixHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
});
