import type { NOT } from "$/logic/not";

describe("NOT", () => {
  it("NOT<1>", () => {
    const _: NOT<1> = false;
    expect(_).toEqual(_);
  });
  it("NOT<0>", () => {
    const _: NOT<0> = true;
    expect(_).toEqual(_);
  });
});
