import type { ConvertMonTypeNoZero } from "$/date/convertMonTypeNoZero";
import type { ConvertMonTypeZero } from "$/date/convertMonTypeZero";
import type { DateType } from "$/date/dateType";
import type { DayType } from "$/date/dayType";
import type { DayTypeInt } from "$/date/dayTypeInt";
import type { MonthsWith31Days } from "$/date/monthsWith31Days";
import type { MonthsWith31DaysInt } from "$/date/monthsWith31DaysInt";
import type { MonthsWithout31Days } from "$/date/monthsWithout31Days";
import type { MonthsWithout31DaysInt } from "$/date/monthsWithout31DaysInt";
import type { MonType } from "$/date/monType";
import type { MonTypeInt } from "$/date/monTypeInt";
import type { MonTypeNoZero } from "$/date/monTypeNoZero";
import type { MonTypeZero } from "$/date/monTypeZero";

describe("dateType Month", () => {
  it("monTypeInt", () => {
    const _: MonTypeInt[] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    expect(_).toEqual(_);
  });
  it("monTypeNoZero", () => {
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
  it("monTypeZero", () => {
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
  it("monType", () => {
    const _: MonType[] = ["1", "01", "9", "09", "12"];
    expect(_).toEqual(_);
  });
  it("monthsWith31DaysInt", () => {
    const _: MonthsWith31DaysInt[] = [1, 3, 5, 7, 8, 10, 12];
    expect(_).toEqual(_);
  });
  it("monthsWith31Days", () => {
    const _: MonthsWith31Days[] = ["1", "3", "5", "7", "8", "10", "12"];
    expect(_).toEqual(_);
  });
  it("monthsWithout31DaysInt", () => {
    const _: MonthsWithout31DaysInt[] = [2, 4, 6, 9, 11];
    expect(_).toEqual(_);
  });
  it("monthsWithout31Days", () => {
    const _: MonthsWithout31Days[] = ["2", "4", "6", "9", "11"];
    expect(_).toEqual(_);
  });
  it("convertMonTypeNoZero", () => {
    const _: [ConvertMonTypeNoZero<"01">, ConvertMonTypeNoZero<"10">] = [
      "1",
      "10",
    ];
    expect(_).toEqual(_);
  });
  it("convertMonTypeZero", () => {
    const _: [
      ConvertMonTypeZero<1>,
      ConvertMonTypeZero<"9">,
      ConvertMonTypeZero<10>,
      ConvertMonTypeZero<"12">,
    ] = ["01", "09", 10, "12"];
    expect(_).toEqual(_);
  });
});

describe("dateType Day", () => {
  it("dayType", () => {
    const _: [
      DayType<"2">[],
      "30" extends DayType<"02"> ? true : false,
      DayType<"4">[],
      "31" extends DayType<"4"> ? true : false,
      DayType<"1">[],
    ] = [["1", "01", "29"], false, ["30"], false, ["30", "31"]];
    expect(_).toEqual(_);
  });
  it("dayTypeInt", () => {
    const _: [
      DayTypeInt<2>[],
      30 extends DayTypeInt<2> ? true : false,
      DayTypeInt<4>[],
      31 extends DayTypeInt<4> ? true : false,
      DayTypeInt<1>[],
    ] = [[1, 29], false, [30], false, [30, 31]];
    expect(_).toEqual(_);
  });
  it("dateType", () => {
    const _: [DateType<"2">[], DateType<2>[], DateType<"12">[]] = [
      ["2024-2-29", "2024/02/01", "2024:02:09"],
      ["2024-2-29", "2024/02/1"],
      ["2024-12-31", "2024/12/01", "2024:12:1"],
    ];
    expect(_).toEqual(_);
  });
});
