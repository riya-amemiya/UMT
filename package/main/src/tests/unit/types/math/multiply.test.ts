import type { MultiHelper, Multiply } from "$/math/multiply";

describe("Multiply", () => {
  it("Multiply<-3, -4>", () => {
    const _: Multiply<-3, -4> = 12;
    expect(_).toEqual(_);
  });
  it("Multiply<-3, 4>", () => {
    const _: Multiply<-3, 4> = -12;
    expect(_).toEqual(_);
  });
  it("Multiply<3, -4>", () => {
    const _: Multiply<3, -4> = -12;
    expect(_).toEqual(_);
  });
  it("Multiply<3, 0>", () => {
    const _: Multiply<3, 0> = 0;
    expect(_).toEqual(_);
  });
  it("Multiply<3, 4>", () => {
    const _: Multiply<3, 4> = 12;
    expect(_).toEqual(_);
  });
  it("MultiHelper<2, 3>", () => {
    const _: MultiHelper<2, 3> = 6;
    expect(_).toEqual(_);
  });
  it("MultiHelper<2, 3, 1>", () => {
    const _: MultiHelper<2, 3, 1> = 7;
    expect(_).toEqual(_);
  });
  it("MultiHelper<2, 0, 5>", () => {
    const _: MultiHelper<2, 0, 5> = 5;
    expect(_).toEqual(_);
  });
});
