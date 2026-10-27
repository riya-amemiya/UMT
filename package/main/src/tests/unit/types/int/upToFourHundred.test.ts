import type { UpToFourHundred } from "$/int/upToFourHundred";

describe("UpToFourHundred", () => {
  it("UpToFourHundred", () => {
    const _: UpToFourHundred[] = [
      390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400,
    ];
    expect(_).toEqual(_);
  });
  it("389 extends UpToFourHundred", () => {
    const _: 389 extends UpToFourHundred ? true : false = false;
    expect(_).toEqual(_);
  });
  it("401 extends UpToFourHundred", () => {
    const _: 401 extends UpToFourHundred ? true : false = false;
    expect(_).toEqual(_);
  });
});
