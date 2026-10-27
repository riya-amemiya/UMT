import type { UpToNineHundred } from "$/int/upToNineHundred";

describe("UpToNineHundred", () => {
  it("UpToNineHundred", () => {
    const _: UpToNineHundred[] = [
      890, 891, 892, 893, 894, 895, 896, 897, 898, 899, 900,
    ];
    expect(_).toEqual(_);
  });
  it("889 extends UpToNineHundred", () => {
    const _: 889 extends UpToNineHundred ? true : false = false;
    expect(_).toEqual(_);
  });
  it("901 extends UpToNineHundred", () => {
    const _: 901 extends UpToNineHundred ? true : false = false;
    expect(_).toEqual(_);
  });
});
