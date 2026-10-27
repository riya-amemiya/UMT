import type { UpToThreeHundredFifty } from "$/int/upToThreeHundredFifty";

describe("UpToThreeHundredFifty", () => {
  it("UpToThreeHundredFifty", () => {
    const _: UpToThreeHundredFifty[] = [
      340, 341, 342, 343, 344, 345, 346, 347, 348, 349, 350,
    ];
    expect(_).toEqual(_);
  });
  it("339 extends UpToThreeHundredFifty", () => {
    const _: 339 extends UpToThreeHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
  it("351 extends UpToThreeHundredFifty", () => {
    const _: 351 extends UpToThreeHundredFifty ? true : false = false;
    expect(_).toEqual(_);
  });
});
