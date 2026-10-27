import type { HoursPmInt } from "$/clock/index";

describe("clock index", () => {
  it("HoursPmInt", () => {
    const _: HoursPmInt[] = [12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23];
    expect(_).toEqual(_);
  });
  it("11 extends HoursPmInt", () => {
    const _: 11 extends HoursPmInt ? true : false = false;
    expect(_).toEqual(_);
  });
});
