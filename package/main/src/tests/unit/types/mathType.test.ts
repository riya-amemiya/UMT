import type { Add } from "$/math/add";
import type { Divide, DivideHelper } from "$/math/divide";
import type { Modulo } from "$/math/modulo";
import type { MultiHelper, Multiply } from "$/math/multiply";
import type { NumberToArray } from "$/math/numberToArray";
import type { Subtract } from "$/math/subtract";

describe("mathType", () => {
  it("numberToArray", () => {
    const _: [NumberToArray<3>, NumberToArray<0>] = [[0, 1, 2], []];
    expect(_).toEqual(_);
  });
  it("add", () => {
    const _: [Add<1, 2>, Add<-1, -2>, Add<-1, 3>, Add<3, -1>] = [3, -3, 2, 2];
    expect(_).toEqual(_);
  });
  it("subtract", () => {
    const _: [
      Subtract<5, 3>,
      Subtract<3, 5>,
      Subtract<-1, -3>,
      Subtract<-1, 2>,
      Subtract<1, -2>,
    ] = [2, -2, 2, -3, 3];
    expect(_).toEqual(_);
  });
  it("multiply", () => {
    const _: [
      Multiply<3, 4>,
      Multiply<-3, -4>,
      Multiply<-3, 4>,
      Multiply<3, -4>,
      Multiply<3, 0>,
    ] = [12, 12, -12, -12, 0];
    expect(_).toEqual(_);
  });
  it("multiHelper", () => {
    const _: [MultiHelper<2, 3>, MultiHelper<2, 3, 1>] = [6, 7];
    expect(_).toEqual(_);
  });
  it("divide", () => {
    const _: [
      Divide<6, 2>,
      Divide<7, 2>,
      Divide<-6, -2>,
      Divide<-6, 2>,
      Divide<6, -2>,
      [Divide<0, 2>] extends [never] ? true : false,
      [Divide<2, 0>] extends [never] ? true : false,
    ] = [3, 3, 3, -3, -3, true, true];
    expect(_).toEqual(_);
  });
  it("divideHelper", () => {
    const _: [DivideHelper<6, 3>, DivideHelper<1, 3>] = [2, 0];
    expect(_).toEqual(_);
  });
  it("modulo", () => {
    const _: [
      Modulo<7, 3>,
      Modulo<6, 3>,
      Modulo<-7, -3>,
      Modulo<-7, 3>,
      Modulo<7, -3>,
    ] = [1, 0, 1, 1, 1];
    expect(_).toEqual(_);
  });
});
