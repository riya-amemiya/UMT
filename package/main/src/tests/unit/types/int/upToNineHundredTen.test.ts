import type { UpToNineHundredTen } from "$/int/upToNineHundredTen";

describe("UpToNineHundredTen", () => {
  it("UpToNineHundredTen", () => {
    const _: UpToNineHundredTen[] = [
      900, 901, 902, 903, 904, 905, 906, 907, 908, 909, 910,
    ];
    expect(_).toEqual(_);
  });
  it("899 extends UpToNineHundredTen", () => {
    const _: 899 extends UpToNineHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
  it("911 extends UpToNineHundredTen", () => {
    const _: 911 extends UpToNineHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
});
