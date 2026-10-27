import type { UpToFourHundredNinety } from "$/int/upToFourHundredNinety";

describe("UpToFourHundredNinety", () => {
  it("UpToFourHundredNinety", () => {
    const _: UpToFourHundredNinety[] = [
      480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490,
    ];
    expect(_).toEqual(_);
  });
  it("479 extends UpToFourHundredNinety", () => {
    const _: 479 extends UpToFourHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
  it("491 extends UpToFourHundredNinety", () => {
    const _: 491 extends UpToFourHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
});
