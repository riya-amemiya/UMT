import type { UpToHundredTwenty } from "$/int/upToHundredTwenty";

describe("UpToHundredTwenty", () => {
  it("UpToHundredTwenty", () => {
    const _: UpToHundredTwenty[] = [
      110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120,
    ];
    expect(_).toEqual(_);
  });
  it("109 extends UpToHundredTwenty", () => {
    const _: 109 extends UpToHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("121 extends UpToHundredTwenty", () => {
    const _: 121 extends UpToHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
});
