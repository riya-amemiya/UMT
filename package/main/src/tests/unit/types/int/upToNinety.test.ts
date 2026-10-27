import type { UpToNinety } from "$/int/upToNinety";

describe("UpToNinety", () => {
  it("UpToNinety", () => {
    const _: UpToNinety[] = [80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90];
    expect(_).toEqual(_);
  });
  it("79 extends UpToNinety", () => {
    const _: 79 extends UpToNinety ? true : false = false;
    expect(_).toEqual(_);
  });
  it("91 extends UpToNinety", () => {
    const _: 91 extends UpToNinety ? true : false = false;
    expect(_).toEqual(_);
  });
});
