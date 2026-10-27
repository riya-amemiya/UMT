import type { UpToSixHundredNinety } from "$/int/upToSixHundredNinety";

describe("UpToSixHundredNinety", () => {
  it("UpToSixHundredNinety", () => {
    const _: UpToSixHundredNinety[] = [
      680, 681, 682, 683, 684, 685, 686, 687, 688, 689, 690,
    ];
    expect(_).toEqual(_);
  });
  it("679 extends UpToSixHundredNinety", () => {
    const _: 679 extends UpToSixHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
  it("691 extends UpToSixHundredNinety", () => {
    const _: 691 extends UpToSixHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
});
