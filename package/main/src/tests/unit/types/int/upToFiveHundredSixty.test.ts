import type { UpToFiveHundredSixty } from "$/int/upToFiveHundredSixty";

describe("UpToFiveHundredSixty", () => {
  it("UpToFiveHundredSixty", () => {
    const _: UpToFiveHundredSixty[] = [
      550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 560,
    ];
    expect(_).toEqual(_);
  });
  it("549 extends UpToFiveHundredSixty", () => {
    const _: 549 extends UpToFiveHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("561 extends UpToFiveHundredSixty", () => {
    const _: 561 extends UpToFiveHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
});
