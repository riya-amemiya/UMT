import type { UpToFourHundredSeventy } from "$/int/upToFourHundredSeventy";

describe("UpToFourHundredSeventy", () => {
  it("UpToFourHundredSeventy", () => {
    const _: UpToFourHundredSeventy[] = [
      460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470,
    ];
    expect(_).toEqual(_);
  });
  it("459 extends UpToFourHundredSeventy", () => {
    const _: 459 extends UpToFourHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
  it("471 extends UpToFourHundredSeventy", () => {
    const _: 471 extends UpToFourHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
});
