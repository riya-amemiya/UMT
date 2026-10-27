import type { Add } from "$/math/add";

describe("Add", () => {
  it("Add<-1, -2>", () => {
    const _: Add<-1, -2> = -3;
    expect(_).toEqual(_);
  });
  it("Add<-1, 3>", () => {
    const _: Add<-1, 3> = 2;
    expect(_).toEqual(_);
  });
  it("Add<3, -1>", () => {
    const _: Add<3, -1> = 2;
    expect(_).toEqual(_);
  });
  it("Add<1, 2>", () => {
    const _: Add<1, 2> = 3;
    expect(_).toEqual(_);
  });
  it("Add<0, 0>", () => {
    const _: Add<0, 0> = 0;
    expect(_).toEqual(_);
  });
});
