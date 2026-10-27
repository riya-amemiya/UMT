import type { DayType } from "$/date/dayType";

describe("DayType", () => {
  it('DayType<"2">', () => {
    const _: DayType<"2">[] = ["1", "9", "01", "09", "10", "19", "20", "29"];
    expect(_).toEqual(_);
  });
  it('DayType<"02">', () => {
    const _: DayType<"02">[] = ["1", "01", "29"];
    expect(_).toEqual(_);
  });
  it('"30" extends DayType<"2">', () => {
    const _: "30" extends DayType<"2"> ? true : false = false;
    expect(_).toEqual(_);
  });
  it('DayType<"4">', () => {
    const _: DayType<"4">[] = [
      "1",
      "9",
      "01",
      "09",
      "10",
      "19",
      "20",
      "29",
      "30",
    ];
    expect(_).toEqual(_);
  });
  it('"31" extends DayType<"4">', () => {
    const _: "31" extends DayType<"4"> ? true : false = false;
    expect(_).toEqual(_);
  });
  it('DayType<"1">', () => {
    const _: DayType<"1">[] = [
      "1",
      "9",
      "01",
      "09",
      "10",
      "19",
      "20",
      "29",
      "30",
      "31",
    ];
    expect(_).toEqual(_);
  });
  it('"32" extends DayType<"1">', () => {
    const _: "32" extends DayType<"1"> ? true : false = false;
    expect(_).toEqual(_);
  });
});
