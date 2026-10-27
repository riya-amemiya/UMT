import type { MillisecondsType } from "$/clock/millisecondsType";

describe("MillisecondsType", () => {
  it("MillisecondsType from Int with two leading zeros", () => {
    const _: MillisecondsType[] = ["000", "009"];
    expect(_).toEqual(_);
  });
  it("MillisecondsType from DoubleDigitInt with a leading zero", () => {
    const _: MillisecondsType[] = ["010", "099"];
    expect(_).toEqual(_);
  });
  it("MillisecondsType from TripleDigitInt", () => {
    const _: MillisecondsType[] = ["100", "999"];
    expect(_).toEqual(_);
  });
  it('"1000" extends MillisecondsType', () => {
    const _: "1000" extends MillisecondsType ? true : false = false;
    expect(_).toEqual(_);
  });
  it('"00" extends MillisecondsType', () => {
    const _: "00" extends MillisecondsType ? true : false = false;
    expect(_).toEqual(_);
  });
});
