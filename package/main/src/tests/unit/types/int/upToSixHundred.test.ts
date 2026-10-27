import type { UpToSixHundred } from "$/int/upToSixHundred";

describe("UpToSixHundred", () => {
  it("UpToSixHundred", () => {
    const _: UpToSixHundred[] = [
      590, 591, 592, 593, 594, 595, 596, 597, 598, 599, 600,
    ];
    expect(_).toEqual(_);
  });
  it("589 extends UpToSixHundred", () => {
    const _: 589 extends UpToSixHundred ? true : false = false;
    expect(_).toEqual(_);
  });
  it("601 extends UpToSixHundred", () => {
    const _: 601 extends UpToSixHundred ? true : false = false;
    expect(_).toEqual(_);
  });
});
