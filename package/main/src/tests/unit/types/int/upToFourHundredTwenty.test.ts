import type { UpToFourHundredTwenty } from "$/int/upToFourHundredTwenty";

describe("UpToFourHundredTwenty", () => {
  it("UpToFourHundredTwenty", () => {
    const _: UpToFourHundredTwenty[] = [
      410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420,
    ];
    expect(_).toEqual(_);
  });
  it("409 extends UpToFourHundredTwenty", () => {
    const _: 409 extends UpToFourHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("421 extends UpToFourHundredTwenty", () => {
    const _: 421 extends UpToFourHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
});
