import type { DayTypeInt } from "$/date/dayTypeInt";

describe("DayTypeInt", () => {
  it("DayTypeInt<2>", () => {
    const _: DayTypeInt<2>[] = [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
      22, 23, 24, 25, 26, 27, 28, 29,
    ];
    expect(_).toEqual(_);
  });
  it("30 extends DayTypeInt<2>", () => {
    const _: 30 extends DayTypeInt<2> ? true : false = false;
    expect(_).toEqual(_);
  });
  it("DayTypeInt<4>", () => {
    const _: DayTypeInt<4>[] = [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
      22, 23, 24, 25, 26, 27, 28, 29, 30,
    ];
    expect(_).toEqual(_);
  });
  it("31 extends DayTypeInt<4>", () => {
    const _: 31 extends DayTypeInt<4> ? true : false = false;
    expect(_).toEqual(_);
  });
  it("DayTypeInt<1>", () => {
    const _: DayTypeInt<1>[] = [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
      22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    ];
    expect(_).toEqual(_);
  });
  it("32 extends DayTypeInt<1>", () => {
    const _: 32 extends DayTypeInt<1> ? true : false = false;
    expect(_).toEqual(_);
  });
});
