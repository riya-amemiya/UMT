import type { UpToThreeHundred } from "$/int/upToThreeHundred";

describe("UpToThreeHundred", () => {
  it("UpToThreeHundred", () => {
    const _: UpToThreeHundred[] = [
      290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300,
    ];
    expect(_).toEqual(_);
  });
  it("289 extends UpToThreeHundred", () => {
    const _: 289 extends UpToThreeHundred ? true : false = false;
    expect(_).toEqual(_);
  });
  it("301 extends UpToThreeHundred", () => {
    const _: 301 extends UpToThreeHundred ? true : false = false;
    expect(_).toEqual(_);
  });
});
