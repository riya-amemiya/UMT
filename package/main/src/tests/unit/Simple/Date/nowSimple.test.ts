import { nowSimple } from "@/Simple/Date/nowSimple";
import type { HoursTypeInt } from "$/clock/hoursTypeInt";
import type { HoursType } from "$/clock/hoursType";

describe("nowSimple", () => {
  beforeEach(() => {
    jest.useFakeTimers();
    jest.setSystemTime(new Date("2024-01-01T12:00:00Z"));
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  test("should return current date with default timezone (9)", () => {
    const result = nowSimple();
    expect(result).toBeInstanceOf(Date);
    expect(result.getUTCHours()).toBe(21);
  });

  test("should handle valid HoursTypeInt values", () => {
    const result0 = nowSimple(0);
    expect(result0.getUTCHours()).toBe(12);

    const result5 = nowSimple(5);
    expect(result5.getUTCHours()).toBe(17);

    const result12 = nowSimple(12);
    expect(result12.getUTCHours()).toBe(0);

    const result23 = nowSimple(23);
    expect(result23.getUTCHours()).toBe(11);
  });

  test("should handle valid HoursType string values", () => {
    const result03 = nowSimple("03");
    expect(result03.getUTCHours()).toBe(15);

    const result00 = nowSimple("00");
    expect(result00.getUTCHours()).toBe(12);

    const result12 = nowSimple("12");
    expect(result12.getUTCHours()).toBe(0);

    const result23 = nowSimple("23");
    expect(result23.getUTCHours()).toBe(11);
  });

  test("should handle all valid HoursTypeInt values", () => {
    const validHours: HoursTypeInt[] = [
      0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 22, 23,
    ];

    for (const hour of validHours) {
      const result = nowSimple(hour);
      expect(result).toBeInstanceOf(Date);
    }
  });

  test("should handle all valid HoursType string values", () => {
    const validStringHours: HoursType[] = [
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

    for (const hour of validStringHours) {
      const result = nowSimple(hour);
      expect(result).toBeInstanceOf(Date);
    }
  });

  test("should handle edge cases with valid types", () => {
    const resultMidnight = nowSimple(0);
    expect(resultMidnight.getUTCHours()).toBe(12);

    const resultNoon = nowSimple(12);
    expect(resultNoon.getUTCHours()).toBe(0);

    const resultLastHour = nowSimple(23);
    expect(resultLastHour.getUTCHours()).toBe(11);
  });

  test("should fallback to default when invalid string is passed", () => {
    // @ts-expect-error Testing invalid input
    const result = nowSimple("invalid");
    expect(result).toBeInstanceOf(Date);
    expect(result.getUTCHours()).toBe(21); // Default timezone 9
  });
});
