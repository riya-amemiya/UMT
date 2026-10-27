import type { MonTypeInt } from "$/date/monTypeInt";

describe("MonTypeInt", () => {
  it("MonTypeInt", () => {
    const _: MonTypeInt[] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    expect(_).toEqual(_);
  });
  it("0 extends MonTypeInt", () => {
    const _: 0 extends MonTypeInt ? true : false = false;
    expect(_).toEqual(_);
  });
  it("13 extends MonTypeInt", () => {
    const _: 13 extends MonTypeInt ? true : false = false;
    expect(_).toEqual(_);
  });
});
