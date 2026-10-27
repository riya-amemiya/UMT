import type { UpToSixHundredTwenty } from "$/int/upToSixHundredTwenty";

describe("UpToSixHundredTwenty", () => {
  it("UpToSixHundredTwenty", () => {
    const _: UpToSixHundredTwenty[] = [
      610, 611, 612, 613, 614, 615, 616, 617, 618, 619, 620,
    ];
    expect(_).toEqual(_);
  });
  it("609 extends UpToSixHundredTwenty", () => {
    const _: 609 extends UpToSixHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("621 extends UpToSixHundredTwenty", () => {
    const _: 621 extends UpToSixHundredTwenty ? true : false = false;
    expect(_).toEqual(_);
  });
});
