import type { UpToSevenHundredFifty } from "$/int/upToSevenHundredFifty";

describe("UpToSevenHundredFifty", () => {
  it("UpToSevenHundredFifty", () => {
    const _: UpToSevenHundredFifty[] = [
      740, 741, 742, 743, 744, 745, 746, 747, 748, 749, 750,
    ];
    expect(_).toEqual(_);
  });
  it("739 extends UpToSevenHundredFifty", () => {
    const _: 739 extends UpToSevenHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("751 extends UpToSevenHundredFifty", () => {
    const _: 751 extends UpToSevenHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
});
