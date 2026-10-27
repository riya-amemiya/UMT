import type { UpToTwoHundredFifty } from "$/int/upToTwoHundredFifty";

describe("UpToTwoHundredFifty", () => {
  it("UpToTwoHundredFifty", () => {
    const _: UpToTwoHundredFifty[] = [
      240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250,
    ];
    expect(_).toEqual(_);
  });
  it("239 extends UpToTwoHundredFifty", () => {
    const _: 239 extends UpToTwoHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("251 extends UpToTwoHundredFifty", () => {
    const _: 251 extends UpToTwoHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
});
