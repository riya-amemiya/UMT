import type { UpToThreeHundredForty } from "$/int/upToThreeHundredForty";

describe("UpToThreeHundredForty", () => {
  it("UpToThreeHundredForty", () => {
    const _: UpToThreeHundredForty[] = [
      330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340,
    ];
    expect(_).toEqual(_);
  });
  it("329 extends UpToThreeHundredForty", () => {
    const _: 329 extends UpToThreeHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("341 extends UpToThreeHundredForty", () => {
    const _: 341 extends UpToThreeHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
});
