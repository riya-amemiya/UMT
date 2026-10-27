import type { UpToSevenHundredEighty } from "$/int/upToSevenHundredEighty";

describe("UpToSevenHundredEighty", () => {
  it("UpToSevenHundredEighty", () => {
    const _: UpToSevenHundredEighty[] = [
      770, 771, 772, 773, 774, 775, 776, 777, 778, 779, 780,
    ];
    expect(_).toEqual(_);
  });
  it("769 extends UpToSevenHundredEighty", () => {
    const _: 769 extends UpToSevenHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("781 extends UpToSevenHundredEighty", () => {
    const _: 781 extends UpToSevenHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
});
