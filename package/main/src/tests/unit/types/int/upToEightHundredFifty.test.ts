import type { UpToEightHundredFifty } from "$/int/upToEightHundredFifty";

describe("UpToEightHundredFifty", () => {
  it("UpToEightHundredFifty", () => {
    const _: UpToEightHundredFifty[] = [
      840, 841, 842, 843, 844, 845, 846, 847, 848, 849, 850,
    ];
    expect(_).toEqual(_);
  });
  it("839 extends UpToEightHundredFifty", () => {
    const _: 839 extends UpToEightHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("851 extends UpToEightHundredFifty", () => {
    const _: 851 extends UpToEightHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
});
