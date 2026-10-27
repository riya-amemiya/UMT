import type { UpToHundredSeventy } from "$/int/upToHundredSeventy";

describe("UpToHundredSeventy", () => {
  it("UpToHundredSeventy", () => {
    const _: UpToHundredSeventy[] = [
      160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170,
    ];
    expect(_).toEqual(_);
  });
  it("159 extends UpToHundredSeventy", () => {
    const _: 159 extends UpToHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
  it("171 extends UpToHundredSeventy", () => {
    const _: 171 extends UpToHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
});
