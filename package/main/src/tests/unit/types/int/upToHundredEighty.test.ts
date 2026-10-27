import type { UpToHundredEighty } from "$/int/upToHundredEighty";

describe("UpToHundredEighty", () => {
  it("UpToHundredEighty", () => {
    const _: UpToHundredEighty[] = [
      170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180,
    ];
    expect(_).toEqual(_);
  });
  it("169 extends UpToHundredEighty", () => {
    const _: 169 extends UpToHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("181 extends UpToHundredEighty", () => {
    const _: 181 extends UpToHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
});
