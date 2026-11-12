import { addBusinessDays } from "@/Date/addBusinessDays";
import { businessDaysBetween } from "@/Date/businessDaysBetween";
import { isSameDay } from "@/Date/isSameDay";

describe("businessDaysBetween", () => {
  it("returns 0 for the same day", () => {
    expect(
      businessDaysBetween(new Date(2025, 3, 21, 9), new Date(2025, 3, 21, 18)),
    ).toBe(0);
  });

  it("excludes the start day and includes the end day", () => {
    expect(
      businessDaysBetween(new Date(2025, 3, 21), new Date(2025, 3, 22)),
    ).toBe(1);
    expect(
      businessDaysBetween(new Date(2025, 3, 21), new Date(2025, 3, 25)),
    ).toBe(4);
  });

  it("skips weekends", () => {
    expect(
      businessDaysBetween(new Date(2025, 3, 18), new Date(2025, 3, 21)),
    ).toBe(1);
    expect(
      businessDaysBetween(new Date(2025, 3, 18), new Date(2025, 3, 20)),
    ).toBe(0);
    expect(
      businessDaysBetween(new Date(2025, 3, 21), new Date(2025, 4, 2)),
    ).toBe(9);
  });

  it("skips holidays", () => {
    expect(
      businessDaysBetween(new Date(2025, 3, 21), new Date(2025, 3, 25), [
        new Date(2025, 3, 23, 15),
      ]),
    ).toBe(3);
  });

  it("returns a negative count when end is before start", () => {
    expect(
      businessDaysBetween(new Date(2025, 3, 21), new Date(2025, 3, 18)),
    ).toBe(-1);
    expect(
      businessDaysBetween(new Date(2025, 3, 25), new Date(2025, 3, 21)),
    ).toBe(-4);
  });

  it("returns 0 rather than -0 for a weekend-only backward range", () => {
    expect(
      Object.is(
        businessDaysBetween(new Date(2025, 3, 20), new Date(2025, 3, 19)),
        0,
      ),
    ).toBe(true);
  });

  it("ignores the time of day", () => {
    expect(
      businessDaysBetween(
        new Date(2025, 3, 21, 23, 59),
        new Date(2025, 3, 22, 0, 1),
      ),
    ).toBe(1);
  });

  it("is the inverse of addBusinessDays", () => {
    const holidays = [new Date(2025, 3, 29)];
    const start = new Date(2025, 3, 18, 9, 30);
    for (const amount of [-12, -5, -1, 1, 3, 7, 15]) {
      const end = addBusinessDays(start, amount, holidays);
      const count = businessDaysBetween(start, end, holidays);
      expect(count).toBe(amount);
      expect(isSameDay(addBusinessDays(start, count, holidays), end)).toBe(
        true,
      );
    }
  });
});
