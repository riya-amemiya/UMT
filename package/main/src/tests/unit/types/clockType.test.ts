import type { HoursAm } from "$/clock/hoursAm";
import type { HoursAmInt } from "$/clock/hoursAmInt";
import type { HoursPm } from "$/clock/hoursPm";
import type { HoursPmInt } from "$/clock/hoursPmInt";
import type { HoursType } from "$/clock/hoursType";
import type { HoursTypeInt } from "$/clock/hoursTypeInt";
import type { MillisecondsType } from "$/clock/millisecondsType";
import type { MillisecondsTypeInt } from "$/clock/millisecondsTypeInt";
import type { MinutesType } from "$/clock/minutesType";
import type { MinutesTypeInt } from "$/clock/minutesTypeInt";
import type { SecondsType } from "$/clock/secondsType";
import type { SecondsTypeInt } from "$/clock/secondsTypeInt";

describe("clockType Hours", () => {
  it("hoursAmInt", () => {
    const _: HoursAmInt[] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    expect(_).toEqual(_);
  });
  it("hoursAm", () => {
    const _: HoursAm[] = [
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
    ];
    expect(_).toEqual(_);
  });
  it("hoursPmInt", () => {
    const _: HoursPmInt[] = [12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23];
    expect(_).toEqual(_);
  });
  it("hoursPm", () => {
    const _: HoursPm[] = [
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
  it("hoursTypeInt", () => {
    const _: HoursTypeInt[] = [
      0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 22, 23,
    ];
    expect(_).toEqual(_);
  });
  it("hoursType", () => {
    const _: HoursType[] = ["00", "09", "12", "23"];
    expect(_).toEqual(_);
  });
});

describe("clockType Minutes", () => {
  it("minutesTypeInt", () => {
    const _: MinutesTypeInt[] = [
      0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
      39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
      57, 58, 59, 60,
    ];
    expect(_).toEqual(_);
  });
  it("minutesType", () => {
    const _: MinutesType[] = [
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
      "24",
      "25",
      "26",
      "27",
      "28",
      "29",
      "30",
      "31",
      "32",
      "33",
      "34",
      "35",
      "36",
      "37",
      "38",
      "39",
      "40",
      "41",
      "42",
      "43",
      "44",
      "45",
      "46",
      "47",
      "48",
      "49",
      "50",
      "51",
      "52",
      "53",
      "54",
      "55",
      "56",
      "57",
      "58",
      "59",
      "60",
    ];
    expect(_).toEqual(_);
  });
});

describe("clockType Seconds", () => {
  it("secondsTypeInt", () => {
    const _: SecondsTypeInt[] = [0, 9, 10, 59, 60];
    expect(_).toEqual(_);
  });
  it("secondsType", () => {
    const _: SecondsType[] = ["00", "09", "10", "59", "60"];
    expect(_).toEqual(_);
  });
});

describe("clockType Milliseconds", () => {
  it("millisecondsTypeInt", () => {
    const _: MillisecondsTypeInt[] = [0, 9, 10, 99, 100, 999];
    expect(_).toEqual(_);
  });
  it("millisecondsType", () => {
    const _: MillisecondsType[] = ["000", "009", "010", "099", "100", "999"];
    expect(_).toEqual(_);
  });
});
