import type { UpToHundredThirty } from "$/int/upToHundredThirty";

describe("UpToHundredThirty", () => {
  it("UpToHundredThirty", () => {
    const _: UpToHundredThirty[] = [
      120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
    ];
    expect(_).toEqual(_);
  });
  it("119 extends UpToHundredThirty", () => {
    const _: 119 extends UpToHundredThirty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("131 extends UpToHundredThirty", () => {
    const _: 131 extends UpToHundredThirty ? true : false = false;
    expect(_).toEqual(_);
  });
});
