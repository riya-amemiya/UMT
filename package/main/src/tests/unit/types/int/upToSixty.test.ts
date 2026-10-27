import type { UpToSixty } from "$/int/upToSixty";

describe("UpToSixty", () => {
  it("UpToSixty", () => {
    const _: UpToSixty[] = [50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60];
    expect(_).toEqual(_);
  });
  it("49 extends UpToSixty", () => {
    const _: 49 extends UpToSixty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("61 extends UpToSixty", () => {
    const _: 61 extends UpToSixty ? true : false = false;
    expect(_).toEqual(_);
  });
});
