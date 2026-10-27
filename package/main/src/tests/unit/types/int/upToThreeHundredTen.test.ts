import type { UpToThreeHundredTen } from "$/int/upToThreeHundredTen";

describe("UpToThreeHundredTen", () => {
  it("UpToThreeHundredTen", () => {
    const _: UpToThreeHundredTen[] = [
      300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310,
    ];
    expect(_).toEqual(_);
  });
  it("299 extends UpToThreeHundredTen", () => {
    const _: 299 extends UpToThreeHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
  it("311 extends UpToThreeHundredTen", () => {
    const _: 311 extends UpToThreeHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
});
