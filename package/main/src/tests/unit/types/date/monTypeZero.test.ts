import type { MonTypeZero } from "$/date/monTypeZero";

describe("MonTypeZero", () => {
  it("MonTypeZero", () => {
    const _: MonTypeZero[] = [
      "01",
      "02",
      "03",
      "04",
      "05",
      "06",
      "07",
      "08",
      "09",
      "10",
      "11",
      "12",
    ];
    expect(_).toEqual(_);
  });
  it('"1" extends MonTypeZero', () => {
    const _: "1" extends MonTypeZero ? true : false = false;
    expect(_).toEqual(_);
  });
  it('"13" extends MonTypeZero', () => {
    const _: "13" extends MonTypeZero ? true : false = false;
    expect(_).toEqual(_);
  });
});
