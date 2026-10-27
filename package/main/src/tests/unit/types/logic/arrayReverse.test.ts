import type { ArrayReverse } from "$/logic/arrayReverse";

describe("ArrayReverse", () => {
  it("ArrayReverse<[1, 2, 3]>", () => {
    const _: ArrayReverse<[1, 2, 3]> = [3, 2, 1];
    expect(_).toEqual(_);
  });
  it("ArrayReverse<[]>", () => {
    const _: ArrayReverse<[]> = [];
    expect(_).toEqual(_);
  });
});
