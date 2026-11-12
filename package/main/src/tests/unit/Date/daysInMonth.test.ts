import { daysInMonth } from "@/Date/daysInMonth";

describe("daysInMonth", () => {
  it("returns 31 for 31-day months", () => {
    for (const month of [0, 2, 4, 6, 7, 9, 11]) {
      expect(daysInMonth(new Date(2025, month, 15))).toBe(31);
    }
  });

  it("returns 30 for 30-day months", () => {
    for (const month of [3, 5, 8, 10]) {
      expect(daysInMonth(new Date(2025, month, 15))).toBe(30);
    }
  });

  it("returns 28 for February in a common year", () => {
    expect(daysInMonth(new Date(2025, 1, 1))).toBe(28);
    expect(daysInMonth(new Date(2100, 1, 1))).toBe(28);
  });

  it("returns 29 for February in a leap year", () => {
    expect(daysInMonth(new Date(2024, 1, 1))).toBe(29);
    expect(daysInMonth(new Date(2000, 1, 1))).toBe(29);
  });

  it("does not depend on the day or time of the input", () => {
    expect(daysInMonth(new Date(2025, 0, 31, 23, 59, 59, 999))).toBe(31);
    expect(daysInMonth(new Date(2024, 1, 29, 12))).toBe(29);
  });

  it("keeps years below 100 as written", () => {
    const date = new Date(0);
    date.setFullYear(0, 1, 1);
    expect(daysInMonth(date)).toBe(29);
  });

  it("does not mutate the input", () => {
    const date = new Date(2025, 0, 31, 10);
    const time = date.getTime();
    daysInMonth(date);
    expect(date.getTime()).toBe(time);
  });

  it("returns NaN for an invalid date", () => {
    expect(daysInMonth(new Date(Number.NaN))).toBeNaN();
  });
});
