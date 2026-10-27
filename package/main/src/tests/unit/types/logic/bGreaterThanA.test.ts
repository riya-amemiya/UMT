import type { BGreaterThanA } from "$/logic/bGreaterThanA";

describe("BGreaterThanA", () => {
  it("BGreaterThanA<-1, -3>", () => {
    const _: BGreaterThanA<-1, -3> = true;
    expect(_).toEqual(_);
  });
  it("BGreaterThanA<-1, 3>", () => {
    const _: BGreaterThanA<-1, 3> = true;
    expect(_).toEqual(_);
  });
  it("BGreaterThanA<3, -1>", () => {
    const _: BGreaterThanA<3, -1> = false;
    expect(_).toEqual(_);
  });
  it("BGreaterThanA<0, 0>", () => {
    const _: BGreaterThanA<0, 0> = false;
    expect(_).toEqual(_);
  });
  it("BGreaterThanA<0, 2>", () => {
    const _: BGreaterThanA<0, 2> = true;
    expect(_).toEqual(_);
  });
  it("BGreaterThanA<2, 0>", () => {
    const _: BGreaterThanA<2, 0> = false;
    expect(_).toEqual(_);
  });
  it("BGreaterThanA<1, 3>", () => {
    const _: BGreaterThanA<1, 3> = true;
    expect(_).toEqual(_);
  });
  it("BGreaterThanA<2, 2>", () => {
    const _: BGreaterThanA<2, 2> = false;
    expect(_).toEqual(_);
  });
});
