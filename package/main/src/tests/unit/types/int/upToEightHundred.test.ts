import type { UpToEightHundred } from "$/int/upToEightHundred";

describe("UpToEightHundred", () => {
  it("UpToEightHundred", () => {
    const _: UpToEightHundred[] = [
      790, 791, 792, 793, 794, 795, 796, 797, 798, 799, 800,
    ];
    expect(_).toEqual(_);
  });
  it("789 extends UpToEightHundred", () => {
    const _: 789 extends UpToEightHundred ? true : false = false;
    expect(_).toEqual(_);
  });
  it("801 extends UpToEightHundred", () => {
    const _: 801 extends UpToEightHundred ? true : false = false;
    expect(_).toEqual(_);
  });
});
