import type { UpToSevenHundredSeventy } from "$/int/upToSevenHundredSeventy";

describe("UpToSevenHundredSeventy", () => {
  it("UpToSevenHundredSeventy", () => {
    const _: UpToSevenHundredSeventy[] = [
      760, 761, 762, 763, 764, 765, 766, 767, 768, 769, 770,
    ];
    expect(_).toEqual(_);
  });
  it("759 extends UpToSevenHundredSeventy", () => {
    const _: 759 extends UpToSevenHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
  it("771 extends UpToSevenHundredSeventy", () => {
    const _: 771 extends UpToSevenHundredSeventy ? true : false = false;
    expect(_).toEqual(_);
  });
});
