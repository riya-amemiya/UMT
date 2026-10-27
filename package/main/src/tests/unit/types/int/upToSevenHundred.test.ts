import type { UpToSevenHundred } from "$/int/upToSevenHundred";

describe("UpToSevenHundred", () => {
  it("UpToSevenHundred", () => {
    const _: UpToSevenHundred[] = [
      690, 691, 692, 693, 694, 695, 696, 697, 698, 699, 700,
    ];
    expect(_).toEqual(_);
  });
  it("689 extends UpToSevenHundred", () => {
    const _: 689 extends UpToSevenHundred ? true : false = false;
    expect(_).toEqual(_);
  });
  it("701 extends UpToSevenHundred", () => {
    const _: 701 extends UpToSevenHundred ? true : false = false;
    expect(_).toEqual(_);
  });
});
