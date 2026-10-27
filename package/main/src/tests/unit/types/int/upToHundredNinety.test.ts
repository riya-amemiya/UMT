import type { UpToHundredNinety } from "$/int/upToHundredNinety";

describe("UpToHundredNinety", () => {
  it("UpToHundredNinety", () => {
    const _: UpToHundredNinety[] = [
      180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190,
    ];
    expect(_).toEqual(_);
  });
  it("179 extends UpToHundredNinety", () => {
    const _: 179 extends UpToHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
  it("191 extends UpToHundredNinety", () => {
    const _: 191 extends UpToHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
});
