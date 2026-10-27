import type { Equal } from "$/logic/equal";

describe("Equal", () => {
  it("Equal<1, 1>", () => {
    const _: Equal<1, 1> = true;
    expect(_).toEqual(_);
  });
  it("Equal<1, 2>", () => {
    const _: Equal<1, 2> = false;
    expect(_).toEqual(_);
  });
});
