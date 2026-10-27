import type { Modulo } from "$/math/modulo";

describe("Modulo", () => {
  it("Modulo<-7, -3>", () => {
    const _: Modulo<-7, -3> = 1;
    expect(_).toEqual(_);
  });
  it("Modulo<-7, 3>", () => {
    const _: Modulo<-7, 3> = 1;
    expect(_).toEqual(_);
  });
  it("Modulo<7, -3>", () => {
    const _: Modulo<7, -3> = 1;
    expect(_).toEqual(_);
  });
  it("Modulo<2, 3>", () => {
    const _: Modulo<2, 3> = 2;
    expect(_).toEqual(_);
  });
  it("Modulo<7, 3>", () => {
    const _: Modulo<7, 3> = 1;
    expect(_).toEqual(_);
  });
  it("Modulo<6, 3>", () => {
    const _: Modulo<6, 3> = 0;
    expect(_).toEqual(_);
  });
});
