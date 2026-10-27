import type { UpToHundredTen } from "$/int/upToHundredTen";

describe("UpToHundredTen", () => {
  it("UpToHundredTen", () => {
    const _: UpToHundredTen[] = [
      100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
    ];
    expect(_).toEqual(_);
  });
  it("99 extends UpToHundredTen", () => {
    const _: 99 extends UpToHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
  it("111 extends UpToHundredTen", () => {
    const _: 111 extends UpToHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
});
