import type { UpToSevenHundredForty } from "$/int/upToSevenHundredForty";

describe("UpToSevenHundredForty", () => {
  it("UpToSevenHundredForty", () => {
    const _: UpToSevenHundredForty[] = [
      730, 731, 732, 733, 734, 735, 736, 737, 738, 739, 740,
    ];
    expect(_).toEqual(_);
  });
  it("729 extends UpToSevenHundredForty", () => {
    const _: 729 extends UpToSevenHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("741 extends UpToSevenHundredForty", () => {
    const _: 741 extends UpToSevenHundredForty ? true : false = false;
    expect(_).toEqual(_);
  });
});
