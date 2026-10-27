import type { UpToFiveHundredEighty } from "$/int/upToFiveHundredEighty";

describe("UpToFiveHundredEighty", () => {
  it("UpToFiveHundredEighty", () => {
    const _: UpToFiveHundredEighty[] = [
      570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 580,
    ];
    expect(_).toEqual(_);
  });
  it("569 extends UpToFiveHundredEighty", () => {
    const _: 569 extends UpToFiveHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("581 extends UpToFiveHundredEighty", () => {
    const _: 581 extends UpToFiveHundredEighty ? true : false = false;
    expect(_).toEqual(_);
  });
});
