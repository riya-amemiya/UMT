import type { NAND } from "$/logic/nand";

describe("NAND", () => {
  it("NAND<1, 1>", () => {
    const _: NAND<1, 1> = false;
    expect(_).toEqual(_);
  });
  it("NAND<1, 0>", () => {
    const _: NAND<1, 0> = true;
    expect(_).toEqual(_);
  });
  it("NAND<0, 1>", () => {
    const _: NAND<0, 1> = true;
    expect(_).toEqual(_);
  });
  it("NAND<0, 0>", () => {
    const _: NAND<0, 0> = true;
    expect(_).toEqual(_);
  });
});
