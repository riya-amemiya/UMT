import type { UpToTwoHundredThirty } from "$/int/upToTwoHundredThirty";

describe("UpToTwoHundredThirty", () => {
  it("UpToTwoHundredThirty", () => {
    const _: UpToTwoHundredThirty[] = [
      220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    ];
    expect(_).toEqual(_);
  });
  it("219 extends UpToTwoHundredThirty", () => {
    const _: 219 extends UpToTwoHundredThirty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("231 extends UpToTwoHundredThirty", () => {
    const _: 231 extends UpToTwoHundredThirty ? true : false = false;
    expect(_).toEqual(_);
  });
});
