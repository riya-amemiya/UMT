import type { UpToFourHundredThirty } from "$/int/upToFourHundredThirty";

describe("UpToFourHundredThirty", () => {
  it("UpToFourHundredThirty", () => {
    const _: UpToFourHundredThirty[] = [
      420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430,
    ];
    expect(_).toEqual(_);
  });
  it("419 extends UpToFourHundredThirty", () => {
    const _: 419 extends UpToFourHundredThirty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("431 extends UpToFourHundredThirty", () => {
    const _: 431 extends UpToFourHundredThirty ? true : false = false;
    expect(_).toEqual(_);
  });
});
