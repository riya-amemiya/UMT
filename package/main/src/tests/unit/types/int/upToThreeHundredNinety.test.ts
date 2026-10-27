import type { UpToThreeHundredNinety } from "$/int/upToThreeHundredNinety";

describe("UpToThreeHundredNinety", () => {
  it("UpToThreeHundredNinety", () => {
    const _: UpToThreeHundredNinety[] = [
      380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390,
    ];
    expect(_).toEqual(_);
  });
  it("379 extends UpToThreeHundredNinety", () => {
    const _: 379 extends UpToThreeHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
  it("391 extends UpToThreeHundredNinety", () => {
    const _: 391 extends UpToThreeHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
});
