import type { UpToSevenHundredNinety } from "$/int/upToSevenHundredNinety";

describe("UpToSevenHundredNinety", () => {
  it("UpToSevenHundredNinety", () => {
    const _: UpToSevenHundredNinety[] = [
      780, 781, 782, 783, 784, 785, 786, 787, 788, 789, 790,
    ];
    expect(_).toEqual(_);
  });
  it("779 extends UpToSevenHundredNinety", () => {
    const _: 779 extends UpToSevenHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
  it("791 extends UpToSevenHundredNinety", () => {
    const _: 791 extends UpToSevenHundredNinety ? true : false = false;
    expect(_).toEqual(_);
  });
});
