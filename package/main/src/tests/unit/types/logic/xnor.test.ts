import type { XNOR } from "$/logic/xnor";

describe("XNOR", () => {
  it("XNOR<1, 1>", () => {
    const _: XNOR<1, 1> = true;
    expect(_).toEqual(_);
  });
  it("XNOR<1, 0>", () => {
    const _: XNOR<1, 0> = false;
    expect(_).toEqual(_);
  });
  it("XNOR<0, 1>", () => {
    const _: XNOR<0, 1> = false;
    expect(_).toEqual(_);
  });
  it("XNOR<0, 0>", () => {
    const _: XNOR<0, 0> = true;
    expect(_).toEqual(_);
  });
});
