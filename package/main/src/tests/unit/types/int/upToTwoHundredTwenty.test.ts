import type { UpToTwoHundredTwenty } from "$/int/upToTwoHundredTwenty";

describe("UpToTwoHundredTwenty", () => {
  it("UpToTwoHundredTwenty", () => {
    const _: UpToTwoHundredTwenty[] = [
      210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
    ];
    expect(_).toEqual(_);
  });
  it("209 extends UpToTwoHundredTwenty", () => {
    const _: 209 extends UpToTwoHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("221 extends UpToTwoHundredTwenty", () => {
    const _: 221 extends UpToTwoHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
});
