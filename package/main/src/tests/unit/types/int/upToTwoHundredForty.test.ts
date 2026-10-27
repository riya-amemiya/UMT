import type { UpToTwoHundredForty } from "$/int/upToTwoHundredForty";

describe("UpToTwoHundredForty", () => {
  it("UpToTwoHundredForty", () => {
    const _: UpToTwoHundredForty[] = [
      230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240,
    ];
    expect(_).toEqual(_);
  });
  it("229 extends UpToTwoHundredForty", () => {
    const _: 229 extends UpToTwoHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("241 extends UpToTwoHundredForty", () => {
    const _: 241 extends UpToTwoHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
});
