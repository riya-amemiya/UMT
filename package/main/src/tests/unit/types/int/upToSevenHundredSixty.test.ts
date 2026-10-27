import type { UpToSevenHundredSixty } from "$/int/upToSevenHundredSixty";

describe("UpToSevenHundredSixty", () => {
  it("UpToSevenHundredSixty", () => {
    const _: UpToSevenHundredSixty[] = [
      750, 751, 752, 753, 754, 755, 756, 757, 758, 759, 760,
    ];
    expect(_).toEqual(_);
  });
  it("749 extends UpToSevenHundredSixty", () => {
    const _: 749 extends UpToSevenHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("761 extends UpToSevenHundredSixty", () => {
    const _: 761 extends UpToSevenHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
});
