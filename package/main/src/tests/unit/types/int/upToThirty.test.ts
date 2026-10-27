import type { UpToThirty } from "$/int/upToThirty";

describe("UpToThirty", () => {
  it("UpToThirty", () => {
    const _: UpToThirty[] = [20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30];
    expect(_).toEqual(_);
  });
  it("19 extends UpToThirty", () => {
    const _: 19 extends UpToThirty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("31 extends UpToThirty", () => {
    const _: 31 extends UpToThirty ? true : false = false;
    expect(_).toEqual(_);
  });
});
