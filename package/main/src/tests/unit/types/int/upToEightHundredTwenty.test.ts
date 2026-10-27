import type { UpToEightHundredTwenty } from "$/int/upToEightHundredTwenty";

describe("UpToEightHundredTwenty", () => {
  it("UpToEightHundredTwenty", () => {
    const _: UpToEightHundredTwenty[] = [
      810, 811, 812, 813, 814, 815, 816, 817, 818, 819, 820,
    ];
    expect(_).toEqual(_);
  });
  it("809 extends UpToEightHundredTwenty", () => {
    const _: 809 extends UpToEightHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("821 extends UpToEightHundredTwenty", () => {
    const _: 821 extends UpToEightHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
});
