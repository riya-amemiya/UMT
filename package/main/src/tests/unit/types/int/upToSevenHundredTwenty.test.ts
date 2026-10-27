import type { UpToSevenHundredTwenty } from "$/int/upToSevenHundredTwenty";

describe("UpToSevenHundredTwenty", () => {
  it("UpToSevenHundredTwenty", () => {
    const _: UpToSevenHundredTwenty[] = [
      710, 711, 712, 713, 714, 715, 716, 717, 718, 719, 720,
    ];
    expect(_).toEqual(_);
  });
  it("709 extends UpToSevenHundredTwenty", () => {
    const _: 709 extends UpToSevenHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("721 extends UpToSevenHundredTwenty", () => {
    const _: 721 extends UpToSevenHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
});
