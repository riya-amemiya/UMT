import type { UpToTwenty } from "$/int/upToTwenty";

describe("UpToTwenty", () => {
  it("UpToTwenty", () => {
    const _: UpToTwenty[] = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    expect(_).toEqual(_);
  });
  it("9 extends UpToTwenty", () => {
    const _: 9 extends UpToTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("21 extends UpToTwenty", () => {
    const _: 21 extends UpToTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
});
