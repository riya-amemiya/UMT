import type { AND } from "$/logic/and";

describe("AND", () => {
  it("AND<1, 1>", () => {
    const _: AND<1, 1> = true;
    expect(_).toEqual(_);
  });
  it("AND<1, 0>", () => {
    const _: AND<1, 0> = false;
    expect(_).toEqual(_);
  });
  it("AND<0, 1>", () => {
    const _: AND<0, 1> = false;
    expect(_).toEqual(_);
  });
  it("AND<0, 0>", () => {
    const _: AND<0, 0> = false;
    expect(_).toEqual(_);
  });
});
