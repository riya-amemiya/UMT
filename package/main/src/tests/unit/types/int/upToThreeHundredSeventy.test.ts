import type { UpToThreeHundredSeventy } from "$/int/upToThreeHundredSeventy";

describe("UpToThreeHundredSeventy", () => {
  it("UpToThreeHundredSeventy", () => {
    const _: UpToThreeHundredSeventy[] = [
      360, 361, 362, 363, 364, 365, 366, 367, 368, 369, 370,
    ];
    expect(_).toEqual(_);
  });
  it("359 extends UpToThreeHundredSeventy", () => {
    const _: 359 extends UpToThreeHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
  it("371 extends UpToThreeHundredSeventy", () => {
    const _: 371 extends UpToThreeHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
});
