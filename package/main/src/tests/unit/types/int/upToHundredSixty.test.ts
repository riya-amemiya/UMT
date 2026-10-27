import type { UpToHundredSixty } from "$/int/upToHundredSixty";

describe("UpToHundredSixty", () => {
  it("UpToHundredSixty", () => {
    const _: UpToHundredSixty[] = [
      150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160,
    ];
    expect(_).toEqual(_);
  });
  it("149 extends UpToHundredSixty", () => {
    const _: 149 extends UpToHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("161 extends UpToHundredSixty", () => {
    const _: 161 extends UpToHundredSixty ? true : false = false;
    expect(_).toEqual(_);
  });
});
