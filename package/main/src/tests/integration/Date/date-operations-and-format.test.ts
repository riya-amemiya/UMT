import { dateRange } from "@/Date/dateRange";
import { format } from "@/Date/format";
import { birthday } from "@/Date/birthday";
import { dayOfWeek } from "@/Date/dayOfWeek";
import { getDay } from "@/Date/getDay";
import { isLeapYear } from "@/Date/isLeapYear";
import { newDateInt } from "@/Date/new";

/**
 * Integration tests for Date utility functions
 *
 * Tests the interaction between date operations and formatting:
 * - Date range generation with formatting
 * - Birthday calculations with day of week
 * - Complex date workflows
 */
describe("Integration test for date operations and formatting", () => {
  it("should generate date range and format each date", () => {
    const startDate = new Date("2025-01-01");
    const endDate = new Date("2025-01-05");
    const dates = dateRange(startDate, endDate);

    const formattedDates = dates.map((date) => format(date, "YYYY-MM-DD"));

    expect(formattedDates).toEqual([
      "2025-01-01",
      "2025-01-02",
      "2025-01-03",
      "2025-01-04",
      "2025-01-05",
    ]);
  });

  it("should calculate age and format birthday with day of week", () => {
    const birthYear = 2000;
    const birthMonth = 3;
    const birthDay = 15;

    const age = birthday(birthYear, birthMonth, birthDay);
    const dayNum = dayOfWeek({
      year: birthYear,
      mon: birthMonth,
      day: birthDay,
    });
    const dayName = getDay(dayNum, "en");

    const birthDate = newDateInt(birthYear, birthMonth, birthDay);
    const formattedBirth = format(birthDate, "MM/DD/YYYY");

    expect(age).toBeGreaterThan(20);
    expect(dayName).toBe("Wed");
    expect(formattedBirth).toBe("03/15/2000");
  });

  it("should generate weekday dates within a month", () => {
    const year = 2025;
    const month = 1;
    const startDate = newDateInt(year, month, 1);
    const endDate = newDateInt(year, month, 31);

    const allDates = dateRange(startDate, endDate);
    const weekdayDates = allDates.filter((date) => {
      const day = date.getDay();
      return day !== 0 && day !== 6;
    });

    const formattedWeekdays = weekdayDates.map((date) => ({
      date: format(date, "YYYY-MM-DD"),
      day: getDay(date.getDay(), "en"),
    }));

    expect(formattedWeekdays.length).toBe(23);
    formattedWeekdays.forEach(({ day }) => {
      expect(["Mon", "Tue", "Wed", "Thu", "Fri"]).toContain(day);
    });
  });

  it("should handle leap year dates correctly", () => {
    const leapYears = [2020, 2024, 2028];
    const nonLeapYears = [2021, 2022, 2023];

    const leapYearFebruaryDates = leapYears.map((year) => {
      const feb1 = newDateInt(year, 2, 1);
      const feb29 = newDateInt(year, 2, 29);
      const dates = dateRange(feb1, feb29);
      return {
        year,
        isLeap: isLeapYear(year),
        febDays: dates.length,
        lastDay: format(dates[dates.length - 1], "YYYY-MM-DD"),
      };
    });

    leapYearFebruaryDates.forEach(({ year, isLeap, febDays, lastDay }) => {
      expect(isLeap).toBe(true);
      expect(febDays).toBe(29);
      expect(lastDay).toBe(`${year}-02-29`);
    });

    nonLeapYears.forEach((year) => {
      expect(isLeapYear(year)).toBe(false);
    });
  });

  it("should create calendar view with formatted dates and day names", () => {
    const year = 2025;
    const month = 4;
    const firstDay = newDateInt(year, month, 1);
    const lastDay = newDateInt(year, month, 30);

    const dates = dateRange(firstDay, lastDay);
    const calendarData = dates.map((date) => ({
      date: format(date, "DD"),
      dayName: getDay(date.getDay(), "en"),
      fullDate: format(date, "YYYY-MM-DD"),
      weekNumber: Math.ceil(date.getDate() / 7),
    }));

    expect(calendarData[0]).toEqual({
      date: "01",
      dayName: "Tue",
      fullDate: "2025-04-01",
      weekNumber: 1,
    });

    expect(calendarData[calendarData.length - 1]).toEqual({
      date: "30",
      dayName: "Wed",
      fullDate: "2025-04-30",
      weekNumber: 5,
    });
  });

  it("should calculate business days between dates", () => {
    const startDate = newDateInt(2025, 1, 1);
    const endDate = newDateInt(2025, 1, 31);

    const allDates = dateRange(startDate, endDate);
    const businessDays = allDates.filter((date) => {
      const dayNum = date.getDay();
      return dayNum !== 0 && dayNum !== 6;
    });

    const formattedBusinessDays = businessDays.map((date) =>
      format(date, "MM/DD"),
    );

    expect(businessDays.length).toBe(23);
    expect(formattedBusinessDays[0]).toBe("01/01");
    expect(formattedBusinessDays[formattedBusinessDays.length - 1]).toBe(
      "01/31",
    );
  });

  it("should handle international date formatting", () => {
    const testDate = newDateInt(2025, 12, 25);

    const formats = {
      us: format(testDate, "MM/DD/YYYY"),
      eu: format(testDate, "DD/MM/YYYY"),
      iso: format(testDate, "YYYY-MM-DD"),
      simple: format(testDate, "MM/DD"),
      dayOfWeekFormat: `${getDay(dayOfWeek({ year: 2025, mon: 12, day: 25 }), "en")}, ${format(testDate, "MM/DD")}`,
    };

    expect(formats.us).toBe("12/25/2025");
    expect(formats.eu).toBe("25/12/2025");
    expect(formats.iso).toBe("2025-12-25");
    expect(formats.simple).toBe("12/25");
    expect(formats.dayOfWeekFormat).toBe("Thu, 12/25");
  });
});
