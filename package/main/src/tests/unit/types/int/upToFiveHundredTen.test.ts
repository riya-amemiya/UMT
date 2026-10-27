import type { UpToFiveHundredTen } from "$/int/upToFiveHundredTen";

describe("UpToFiveHundredTen", () => {
  it("UpToFiveHundredTen", () => {
    const _: UpToFiveHundredTen[] = [
      500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510,
    ];
    expect(_).toEqual(_);
  });
  it("499 extends UpToFiveHundredTen", () => {
    const _: 499 extends UpToFiveHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
  it("511 extends UpToFiveHundredTen", () => {
    const _: 511 extends UpToFiveHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
});
