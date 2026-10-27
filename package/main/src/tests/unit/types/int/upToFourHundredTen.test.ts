import type { UpToFourHundredTen } from "$/int/upToFourHundredTen";

describe("UpToFourHundredTen", () => {
  it("UpToFourHundredTen", () => {
    const _: UpToFourHundredTen[] = [
      400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410,
    ];
    expect(_).toEqual(_);
  });
  it("399 extends UpToFourHundredTen", () => {
    const _: 399 extends UpToFourHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
  it("411 extends UpToFourHundredTen", () => {
    const _: 411 extends UpToFourHundredTen ? true : false = false;
    expect(_).toEqual(_);
  });
});
