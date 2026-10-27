import type { UpToSixHundredTen } from "$/int/upToSixHundredTen";

describe("UpToSixHundredTen", () => {
  it("UpToSixHundredTen", () => {
    const _: UpToSixHundredTen[] = [
      600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610,
    ];
    expect(_).toEqual(_);
  });
  it("599 extends UpToSixHundredTen", () => {
    const _: 599 extends UpToSixHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
  it("611 extends UpToSixHundredTen", () => {
    const _: 611 extends UpToSixHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
});
