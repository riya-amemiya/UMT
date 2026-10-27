import type { IMPLY } from "$/logic/imply";

describe("IMPLY", () => {
  it("IMPLY<1, 1>", () => {
    const _: IMPLY<1, 1> = true;
    expect(_).toEqual(_);
  });
  it("IMPLY<1, 0>", () => {
    const _: IMPLY<1, 0> = false;
    expect(_).toEqual(_);
  });
  it("IMPLY<0, 1>", () => {
    const _: IMPLY<0, 1> = true;
    expect(_).toEqual(_);
  });
  it("IMPLY<0, 0>", () => {
    const _: IMPLY<0, 0> = true;
    expect(_).toEqual(_);
  });
});
