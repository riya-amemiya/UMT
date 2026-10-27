import type { ZeroAorB } from "$/logic/zeroAorB";

describe("ZeroAorB", () => {
  it("ZeroAorB<0, 1>", () => {
    const _: ZeroAorB<0, 1> = true;
    expect(_).toEqual(_);
  });
  it("ZeroAorB<1, 0>", () => {
    const _: ZeroAorB<1, 0> = true;
    expect(_).toEqual(_);
  });
  it("ZeroAorB<1, 2>", () => {
    const _: ZeroAorB<1, 2> = false;
    expect(_).toEqual(_);
  });
});
