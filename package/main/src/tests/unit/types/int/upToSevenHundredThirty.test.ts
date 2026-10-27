import type { UpToSevenHundredThirty } from "$/int/upToSevenHundredThirty";

describe("UpToSevenHundredThirty", () => {
  it("UpToSevenHundredThirty", () => {
    const _: UpToSevenHundredThirty[] = [
      720, 721, 722, 723, 724, 725, 726, 727, 728, 729, 730,
    ];
    expect(_).toEqual(_);
  });
  it("719 extends UpToSevenHundredThirty", () => {
    const _: 719 extends UpToSevenHundredThirty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("731 extends UpToSevenHundredThirty", () => {
    const _: 731 extends UpToSevenHundredThirty ? true : false = false;
    expect(_).toEqual(_);
  });
});
