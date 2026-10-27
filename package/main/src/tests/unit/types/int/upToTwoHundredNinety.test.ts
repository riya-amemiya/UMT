import type { UpToTwoHundredNinety } from "$/int/upToTwoHundredNinety";

describe("UpToTwoHundredNinety", () => {
  it("UpToTwoHundredNinety", () => {
    const _: UpToTwoHundredNinety[] = [
      280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291,
    ];
    expect(_).toEqual(_);
  });
  it("279 extends UpToTwoHundredNinety", () => {
    const _: 279 extends UpToTwoHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
  it("292 extends UpToTwoHundredNinety", () => {
    const _: 292 extends UpToTwoHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
});
