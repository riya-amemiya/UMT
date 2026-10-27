import type { MonType } from "$/date/monType";

describe("MonType", () => {
  it("MonType", () => {
    const _: MonType[] = [
      "1",
      "2",
      "3",
      "4",
      "5",
      "6",
      "7",
      "8",
      "9",
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
  it('"13" extends MonType', () => {
    const _: "13" extends MonType ? true : false = false;
    expect(_).toEqual(_);
  });
  it('"00" extends MonType', () => {
    const _: "00" extends MonType ? true : false = false;
    expect(_).toEqual(_);
  });
});
