import type { Subtract } from "$/math/subtract";

describe("Subtract", () => {
  it("Subtract<-1, -3>", () => {
    const _: Subtract<-1, -3> = 2;
    expect(_).toEqual(_);
  });
  it("Subtract<-1, 2>", () => {
    const _: Subtract<-1, 2> = -3;
    expect(_).toEqual(_);
  });
  it("Subtract<1, -2>", () => {
    const _: Subtract<1, -2> = 3;
    expect(_).toEqual(_);
  });
  it("Subtract<5, 3>", () => {
    const _: Subtract<5, 3> = 2;
    expect(_).toEqual(_);
  });
  it("Subtract<3, 3>", () => {
    const _: Subtract<3, 3> = 0;
    expect(_).toEqual(_);
  });
  it("Subtract<3, 5>", () => {
    const _: Subtract<3, 5> = -2;
    expect(_).toEqual(_);
  });
});
