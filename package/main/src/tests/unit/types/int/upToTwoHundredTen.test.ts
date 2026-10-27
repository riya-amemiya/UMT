import type { UpToTwoHundredTen } from "$/int/upToTwoHundredTen";

describe("UpToTwoHundredTen", () => {
  it("UpToTwoHundredTen", () => {
    const _: UpToTwoHundredTen[] = [
      200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
    ];
    expect(_).toEqual(_);
  });
  it("199 extends UpToTwoHundredTen", () => {
    const _: 199 extends UpToTwoHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
  it("211 extends UpToTwoHundredTen", () => {
    const _: 211 extends UpToTwoHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
});
