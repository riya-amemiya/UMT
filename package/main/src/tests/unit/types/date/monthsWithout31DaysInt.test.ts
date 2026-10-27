import type { MonthsWithout31DaysInt } from "$/date/monthsWithout31DaysInt";

describe("MonthsWithout31DaysInt", () => {
  it("MonthsWithout31DaysInt", () => {
    const _: MonthsWithout31DaysInt[] = [2, 4, 6, 9, 11];
    expect(_).toEqual(_);
  });
  it("1 extends MonthsWithout31DaysInt", () => {
    const _: 1 extends MonthsWithout31DaysInt ? true : false = false;
    expect(_).toEqual(_);
  });
});
