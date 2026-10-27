import type { UpToForty } from "$/int/upToForty";

describe("UpToForty", () => {
  it("UpToForty", () => {
    const _: UpToForty[] = [30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40];
    expect(_).toEqual(_);
  });
  it("29 extends UpToForty", () => {
    const _: 29 extends UpToForty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("41 extends UpToForty", () => {
    const _: 41 extends UpToForty ? true : false = false;
    expect(_).toEqual(_);
  });
});
