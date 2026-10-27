import type { MonthsWith31Days } from "$/date/monthsWith31Days";

describe("MonthsWith31Days", () => {
  it("MonthsWith31Days", () => {
    const _: MonthsWith31Days[] = ["1", "3", "5", "7", "8", "10", "12"];
    expect(_).toEqual(_);
  });
  it('"2" extends MonthsWith31Days', () => {
    const _: "2" extends MonthsWith31Days ? true : false = false;
    expect(_).toEqual(_);
  });
});
