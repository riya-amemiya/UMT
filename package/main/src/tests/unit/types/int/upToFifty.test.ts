import type { UpToFifty } from "$/int/upToFifty";

describe("UpToFifty", () => {
  it("UpToFifty", () => {
    const _: UpToFifty[] = [40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50];
    expect(_).toEqual(_);
  });
  it("39 extends UpToFifty", () => {
    const _: 39 extends UpToFifty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("51 extends UpToFifty", () => {
    const _: 51 extends UpToFifty ? true : false = false;
    expect(_).toEqual(_);
  });
});
