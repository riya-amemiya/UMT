import type { UpToFiveHundred } from "$/int/upToFiveHundred";

describe("UpToFiveHundred", () => {
  it("UpToFiveHundred", () => {
    const _: UpToFiveHundred[] = [
      490, 491, 492, 493, 494, 495, 496, 497, 498, 499, 500,
    ];
    expect(_).toEqual(_);
  });
  it("489 extends UpToFiveHundred", () => {
    const _: 489 extends UpToFiveHundred ? true : false = false;
    expect(_).toEqual(_);
  });
  it("501 extends UpToFiveHundred", () => {
    const _: 501 extends UpToFiveHundred ? true : false = false;
    expect(_).toEqual(_);
  });
});
