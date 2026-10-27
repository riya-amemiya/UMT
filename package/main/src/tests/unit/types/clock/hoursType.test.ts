import type { HoursType } from "$/clock/hoursType";

describe("HoursType", () => {
  it("HoursType", () => {
    const _: HoursType[] = [
      "00",
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
      "13",
      "14",
      "15",
      "16",
      "17",
      "18",
      "19",
      "20",
      "21",
      "22",
      "23",
    ];
    expect(_).toEqual(_);
  });
  it('"24" extends HoursType', () => {
    const _: "24" extends HoursType ? true : false = false;
    expect(_).toEqual(_);
  });
  it('"0" extends HoursType', () => {
    const _: "0" extends HoursType ? true : false = false;
    expect(_).toEqual(_);
  });
});
