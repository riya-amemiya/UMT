import type { OR } from "$/logic/or";

describe("OR", () => {
  it("OR<1, 1>", () => {
    const _: OR<1, 1> = true;
    expect(_).toEqual(_);
  });
  it("OR<1, 0>", () => {
    const _: OR<1, 0> = true;
    expect(_).toEqual(_);
  });
  it("OR<0, 1>", () => {
    const _: OR<0, 1> = true;
    expect(_).toEqual(_);
  });
  it("OR<0, 0>", () => {
    const _: OR<0, 0> = false;
    expect(_).toEqual(_);
  });
});
