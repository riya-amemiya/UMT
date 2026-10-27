import type { UpToNineHundredTwenty } from "$/int/upToNineHundredTwenty";

describe("UpToNineHundredTwenty", () => {
  it("UpToNineHundredTwenty", () => {
    const _: UpToNineHundredTwenty[] = [
      910, 911, 912, 913, 914, 915, 916, 917, 918, 919, 920,
    ];
    expect(_).toEqual(_);
  });
  it("909 extends UpToNineHundredTwenty", () => {
    const _: 909 extends UpToNineHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("921 extends UpToNineHundredTwenty", () => {
    const _: 921 extends UpToNineHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
});
