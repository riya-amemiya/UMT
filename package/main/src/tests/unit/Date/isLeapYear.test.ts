import { isLeapYear } from "@/Date/isLeapYear";

describe("isLeapYear", () => {
  it("should return true for years divisible by 4 but not by 100", () => {
    expect(isLeapYear(2020)).toBe(true);
    expect(isLeapYear(2024)).toBe(true);
    expect(isLeapYear(1996)).toBe(true);
    expect(isLeapYear(2004)).toBe(true);
    expect(isLeapYear(2008)).toBe(true);
  });

  it("should return false for years divisible by 100 but not by 400", () => {
    expect(isLeapYear(1900)).toBe(false);
    expect(isLeapYear(2100)).toBe(false);
    expect(isLeapYear(1700)).toBe(false);
    expect(isLeapYear(1800)).toBe(false);
    expect(isLeapYear(2200)).toBe(false);
  });

  it("should return true for years divisible by 400", () => {
    expect(isLeapYear(2000)).toBe(true);
    expect(isLeapYear(1600)).toBe(true);
    expect(isLeapYear(2400)).toBe(true);
    expect(isLeapYear(800)).toBe(true);
    expect(isLeapYear(1200)).toBe(true);
  });

  it("should return false for non-leap years", () => {
    expect(isLeapYear(2023)).toBe(false);
    expect(isLeapYear(2025)).toBe(false);
    expect(isLeapYear(1997)).toBe(false);
    expect(isLeapYear(2001)).toBe(false);
    expect(isLeapYear(2003)).toBe(false);
  });

  it("should handle early years", () => {
    expect(isLeapYear(4)).toBe(true);
    expect(isLeapYear(100)).toBe(false);
    expect(isLeapYear(400)).toBe(true);
    expect(isLeapYear(8)).toBe(true);
    expect(isLeapYear(1)).toBe(false);
  });

  it("should handle edge cases and boundary values", () => {
    expect(isLeapYear(0)).toBe(true);
    expect(isLeapYear(-4)).toBe(true);
    expect(isLeapYear(-100)).toBe(false);
    expect(isLeapYear(-400)).toBe(true);
  });

  it("should handle decimal numbers", () => {
    expect(isLeapYear(2020.5)).toBe(false);
    expect(isLeapYear(2000.1)).toBe(false);
    expect(isLeapYear(1999.9)).toBe(false);
  });

  it("should handle very large years", () => {
    expect(isLeapYear(4000)).toBe(true);
    expect(isLeapYear(8000)).toBe(true);
    expect(isLeapYear(9999)).toBe(false);
    expect(isLeapYear(10000)).toBe(true);
  });

  it("should verify leap year pattern", () => {
    const recentLeapYears = [1996, 2000, 2004, 2008, 2012, 2016, 2020, 2024];
    const recentNonLeapYears = [
      1997, 1998, 1999, 2001, 2002, 2003, 2005, 2006, 2007, 2009, 2010, 2011,
      2013, 2014, 2015, 2017, 2018, 2019, 2021, 2022, 2023,
    ];

    recentLeapYears.forEach((year) => {
      expect(isLeapYear(year)).toBe(true);
    });

    recentNonLeapYears.forEach((year) => {
      expect(isLeapYear(year)).toBe(false);
    });
  });
});
