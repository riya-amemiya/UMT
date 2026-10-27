import type { UpToSixHundredFifty } from "$/int/upToSixHundredFifty";

describe("UpToSixHundredFifty", () => {
  it("UpToSixHundredFifty", () => {
    const _: UpToSixHundredFifty[] = [
      640, 641, 642, 643, 644, 645, 646, 647, 648, 649, 650,
    ];
    expect(_).toEqual(_);
  });
  it("639 extends UpToSixHundredFifty", () => {
    const _: 639 extends UpToSixHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("651 extends UpToSixHundredFifty", () => {
    const _: 651 extends UpToSixHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
});
