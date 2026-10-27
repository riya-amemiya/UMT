import type { UpToTwoHundred } from "$/int/upToTwoHundred";

describe("UpToTwoHundred", () => {
  it("UpToTwoHundred", () => {
    const _: UpToTwoHundred[] = [
      190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200,
    ];
    expect(_).toEqual(_);
  });
  it("189 extends UpToTwoHundred", () => {
    const _: 189 extends UpToTwoHundred ? true : false = false;
    expect(_).toEqual(_);
  });
  it("201 extends UpToTwoHundred", () => {
    const _: 201 extends UpToTwoHundred ? true : false = false;
    expect(_).toEqual(_);
  });
});
