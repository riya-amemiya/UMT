import type { UpToEightHundredSixty } from "$/int/upToEightHundredSixty";

describe("UpToEightHundredSixty", () => {
  it("UpToEightHundredSixty", () => {
    const _: UpToEightHundredSixty[] = [
      850, 851, 852, 853, 854, 855, 856, 857, 858, 859, 860,
    ];
    expect(_).toEqual(_);
  });
  it("849 extends UpToEightHundredSixty", () => {
    const _: 849 extends UpToEightHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("861 extends UpToEightHundredSixty", () => {
    const _: 861 extends UpToEightHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
});
