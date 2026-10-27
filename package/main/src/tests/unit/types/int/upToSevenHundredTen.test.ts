import type { UpToSevenHundredTen } from "$/int/upToSevenHundredTen";

describe("UpToSevenHundredTen", () => {
  it("UpToSevenHundredTen", () => {
    const _: UpToSevenHundredTen[] = [
      700, 701, 702, 703, 704, 705, 706, 707, 708, 709, 710,
    ];
    expect(_).toEqual(_);
  });
  it("699 extends UpToSevenHundredTen", () => {
    const _: 699 extends UpToSevenHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
  it("711 extends UpToSevenHundredTen", () => {
    const _: 711 extends UpToSevenHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
});
