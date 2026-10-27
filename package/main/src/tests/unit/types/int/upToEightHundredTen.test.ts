import type { UpToEightHundredTen } from "$/int/upToEightHundredTen";

describe("UpToEightHundredTen", () => {
  it("UpToEightHundredTen", () => {
    const _: UpToEightHundredTen[] = [
      800, 801, 802, 803, 804, 805, 806, 807, 808, 809, 810,
    ];
    expect(_).toEqual(_);
  });
  it("799 extends UpToEightHundredTen", () => {
    const _: 799 extends UpToEightHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
  it("811 extends UpToEightHundredTen", () => {
    const _: 811 extends UpToEightHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
});
