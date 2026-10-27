import type { NumberToArray } from "$/math/numberToArray";

describe("NumberToArray", () => {
  it("NumberToArray<3>", () => {
    const _: NumberToArray<3> = [0, 1, 2];
    expect(_).toEqual(_);
  });
  it("NumberToArray<0>", () => {
    const _: NumberToArray<0> = [];
    expect(_).toEqual(_);
  });
});
