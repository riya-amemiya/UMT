import type { UpToTwoHundredSixty } from "$/int/upToTwoHundredSixty";

describe("UpToTwoHundredSixty", () => {
  it("UpToTwoHundredSixty", () => {
    const _: UpToTwoHundredSixty[] = [
      250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261,
    ];
    expect(_).toEqual(_);
  });
  it("249 extends UpToTwoHundredSixty", () => {
    const _: 249 extends UpToTwoHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("262 extends UpToTwoHundredSixty", () => {
    const _: 262 extends UpToTwoHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
});
