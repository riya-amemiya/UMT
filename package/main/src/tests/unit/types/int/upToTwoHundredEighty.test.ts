import type { UpToTwoHundredEighty } from "$/int/upToTwoHundredEighty";

describe("UpToTwoHundredEighty", () => {
  it("UpToTwoHundredEighty", () => {
    const _: UpToTwoHundredEighty[] = [
      270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281,
    ];
    expect(_).toEqual(_);
  });
  it("269 extends UpToTwoHundredEighty", () => {
    const _: 269 extends UpToTwoHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("282 extends UpToTwoHundredEighty", () => {
    const _: 282 extends UpToTwoHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
});
