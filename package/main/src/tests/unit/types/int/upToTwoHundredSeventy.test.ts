import type { UpToTwoHundredSeventy } from "$/int/upToTwoHundredSeventy";

describe("UpToTwoHundredSeventy", () => {
  it("UpToTwoHundredSeventy", () => {
    const _: UpToTwoHundredSeventy[] = [
      260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271,
    ];
    expect(_).toEqual(_);
  });
  it("259 extends UpToTwoHundredSeventy", () => {
    const _: 259 extends UpToTwoHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
  it("272 extends UpToTwoHundredSeventy", () => {
    const _: 272 extends UpToTwoHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
});
