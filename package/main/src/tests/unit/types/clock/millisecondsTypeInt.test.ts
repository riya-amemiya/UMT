import type { MillisecondsTypeInt } from "$/clock/millisecondsTypeInt";

describe("MillisecondsTypeInt", () => {
  it("MillisecondsTypeInt from Int", () => {
    const _: MillisecondsTypeInt[] = [0, 9];
    expect(_).toEqual(_);
  });
  it("MillisecondsTypeInt from DoubleDigitInt", () => {
    const _: MillisecondsTypeInt[] = [10, 99];
    expect(_).toEqual(_);
  });
  it("MillisecondsTypeInt from TripleDigitInt", () => {
    const _: MillisecondsTypeInt[] = [100, 999];
    expect(_).toEqual(_);
  });
  it("1000 extends MillisecondsTypeInt", () => {
    const _: 1000 extends MillisecondsTypeInt ? true : false = false;
    expect(_).toEqual(_);
  });
});
