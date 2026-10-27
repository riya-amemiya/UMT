import type { UpToHundredFifty } from "$/int/upToHundredFifty";

describe("UpToHundredFifty", () => {
  it("UpToHundredFifty", () => {
    const _: UpToHundredFifty[] = [
      140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150,
    ];
    expect(_).toEqual(_);
  });
  it("139 extends UpToHundredFifty", () => {
    const _: 139 extends UpToHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("151 extends UpToHundredFifty", () => {
    const _: 151 extends UpToHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
});
