import type { UpToFiveHundredThirty } from "$/int/upToFiveHundredThirty";

describe("UpToFiveHundredThirty", () => {
  it("UpToFiveHundredThirty", () => {
    const _: UpToFiveHundredThirty[] = [
      520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530,
    ];
    expect(_).toEqual(_);
  });
  it("519 extends UpToFiveHundredThirty", () => {
    const _: 519 extends UpToFiveHundredThirty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("531 extends UpToFiveHundredThirty", () => {
    const _: 531 extends UpToFiveHundredThirty ? true : false = false;
    expect(_).toEqual(_);
  });
});
