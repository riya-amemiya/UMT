import type { UpToThreeHundredEighty } from "$/int/upToThreeHundredEighty";

describe("UpToThreeHundredEighty", () => {
  it("UpToThreeHundredEighty", () => {
    const _: UpToThreeHundredEighty[] = [
      370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380,
    ];
    expect(_).toEqual(_);
  });
  it("369 extends UpToThreeHundredEighty", () => {
    const _: 369 extends UpToThreeHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("381 extends UpToThreeHundredEighty", () => {
    const _: 381 extends UpToThreeHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
});
