import type { MonthsWithout31Days } from "$/date/monthsWithout31Days";

describe("MonthsWithout31Days", () => {
  it("MonthsWithout31Days", () => {
    const _: MonthsWithout31Days[] = ["2", "4", "6", "9", "11"];
    expect(_).toEqual(_);
  });
  it('"1" extends MonthsWithout31Days', () => {
    const _: "1" extends MonthsWithout31Days ? true : false = false;
    expect(_).toEqual(_);
  });
});
