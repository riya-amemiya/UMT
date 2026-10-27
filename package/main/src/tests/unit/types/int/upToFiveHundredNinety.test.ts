import type { UpToFiveHundredNinety } from "$/int/upToFiveHundredNinety";

describe("UpToFiveHundredNinety", () => {
  it("UpToFiveHundredNinety", () => {
    const _: UpToFiveHundredNinety[] = [
      580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590,
    ];
    expect(_).toEqual(_);
  });
  it("579 extends UpToFiveHundredNinety", () => {
    const _: 579 extends UpToFiveHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
  it("591 extends UpToFiveHundredNinety", () => {
    const _: 591 extends UpToFiveHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
});
