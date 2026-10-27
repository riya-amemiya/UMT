import type { UpToSeventy } from "$/int/upToSeventy";

describe("UpToSeventy", () => {
  it("UpToSeventy", () => {
    const _: UpToSeventy[] = [60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70];
    expect(_).toEqual(_);
  });
  it("59 extends UpToSeventy", () => {
    const _: 59 extends UpToSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
  it("71 extends UpToSeventy", () => {
    const _: 71 extends UpToSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
});
