import type { MonthsWith31DaysInt } from "$/date/monthsWith31DaysInt";

describe("MonthsWith31DaysInt", () => {
  it("MonthsWith31DaysInt", () => {
    const _: MonthsWith31DaysInt[] = [1, 3, 5, 7, 8, 10, 12];
    expect(_).toEqual(_);
  });
  it("2 extends MonthsWith31DaysInt", () => {
    const _: 2 extends MonthsWith31DaysInt ? true : false = false;
    expect(_).toEqual(_);
  });
});
