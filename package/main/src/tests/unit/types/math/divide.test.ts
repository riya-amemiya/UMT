import type { Divide, DivideHelper } from "$/math/divide";

describe("Divide", () => {
  it("Divide<0, 2>", () => {
    const _: [Divide<0, 2>] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
  it("Divide<2, 0>", () => {
    const _: [Divide<2, 0>] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
  it("Divide<-6, -2>", () => {
    const _: Divide<-6, -2> = 3;
    expect(_).toEqual(_);
  });
  it("Divide<-6, 2>", () => {
    const _: Divide<-6, 2> = -3;
    expect(_).toEqual(_);
  });
  it("Divide<6, -2>", () => {
    const _: Divide<6, -2> = -3;
    expect(_).toEqual(_);
  });
  it("Divide<6, 2>", () => {
    const _: Divide<6, 2> = 3;
    expect(_).toEqual(_);
  });
  it("Divide<7, 2>", () => {
    const _: Divide<7, 2> = 3;
    expect(_).toEqual(_);
  });
  it("DivideHelper<1, 3>", () => {
    const _: DivideHelper<1, 3> = 0;
    expect(_).toEqual(_);
  });
  it("DivideHelper<6, 3>", () => {
    const _: DivideHelper<6, 3> = 2;
    expect(_).toEqual(_);
  });
  it("DivideHelper<6, 3, 1>", () => {
    const _: DivideHelper<6, 3, 1> = 3;
    expect(_).toEqual(_);
  });
});
