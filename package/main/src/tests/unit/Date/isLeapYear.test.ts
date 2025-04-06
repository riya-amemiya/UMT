import { isLeapYear } from "@/Date/isLeapYear";

describe("isLeapYear", () => {
  it("should return true for years divisible by 4 but not by 100", () => {
    expect(isLeapYear(2020)).toBe(true);
    expect(isLeapYear(2024)).toBe(true);
    expect(isLeapYear(1996)).toBe(true);
  });

  it("should return false for years divisible by 100 but not by 400", () => {
    expect(isLeapYear(1900)).toBe(false);
    expect(isLeapYear(2100)).toBe(false);
    expect(isLeapYear(1700)).toBe(false);
  });

  it("should return true for years divisible by 400", () => {
    expect(isLeapYear(2000)).toBe(true);
    expect(isLeapYear(1600)).toBe(true);
    expect(isLeapYear(2400)).toBe(true);
  });

  it("should return false for non-leap years", () => {
    expect(isLeapYear(2023)).toBe(false);
    expect(isLeapYear(2025)).toBe(false);
    expect(isLeapYear(1997)).toBe(false);
  });

  it("should handle early years", () => {
    expect(isLeapYear(4)).toBe(true);
    expect(isLeapYear(100)).toBe(false);
    expect(isLeapYear(400)).toBe(true);
  });
});
