import type { UpToNineHundredSeventy } from "$/int/upToNineHundredSeventy";

describe("UpToNineHundredSeventy", () => {
  it("UpToNineHundredSeventy", () => {
    const _: UpToNineHundredSeventy[] = [
      960, 961, 962, 963, 964, 965, 966, 967, 968, 969, 970,
    ];
    expect(_).toEqual(_);
  });
  it("959 extends UpToNineHundredSeventy", () => {
    const _: 959 extends UpToNineHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
  it("971 extends UpToNineHundredSeventy", () => {
    const _: 971 extends UpToNineHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
});
