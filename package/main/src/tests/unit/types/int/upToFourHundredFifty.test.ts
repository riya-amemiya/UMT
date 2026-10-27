import type { UpToFourHundredFifty } from "$/int/upToFourHundredFifty";

describe("UpToFourHundredFifty", () => {
  it("UpToFourHundredFifty", () => {
    const _: UpToFourHundredFifty[] = [
      440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450,
    ];
    expect(_).toEqual(_);
  });
  it("439 extends UpToFourHundredFifty", () => {
    const _: 439 extends UpToFourHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("451 extends UpToFourHundredFifty", () => {
    const _: 451 extends UpToFourHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
});
