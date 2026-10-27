import type { NumberToArray } from "$/math/index";

describe("math index", () => {
  it("NumberToArray<2>", () => {
    const _: NumberToArray<2> = [0, 1];
    expect(_).toEqual(_);
  });
  it("NumberToArray<0>", () => {
    const _: NumberToArray<0> = [];
    expect(_).toEqual(_);
  });
});
