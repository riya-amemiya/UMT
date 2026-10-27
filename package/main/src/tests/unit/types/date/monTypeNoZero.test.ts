import type { MonTypeNoZero } from "$/date/monTypeNoZero";

describe("MonTypeNoZero", () => {
  it("MonTypeNoZero", () => {
    const _: MonTypeNoZero[] = [
      "1",
      "2",
      "3",
      "4",
      "5",
      "6",
      "7",
      "8",
      "9",
      "10",
      "11",
      "12",
    ];
    expect(_).toEqual(_);
  });
  it('"01" extends MonTypeNoZero', () => {
    const _: "01" extends MonTypeNoZero ? true : false = false;
    expect(_).toEqual(_);
  });
  it('"13" extends MonTypeNoZero', () => {
    const _: "13" extends MonTypeNoZero ? true : false = false;
    expect(_).toEqual(_);
  });
});
