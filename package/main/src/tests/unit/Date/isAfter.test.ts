import { isAfter } from "@/Date/isAfter";

describe("isAfter", () => {
  it("returns true when the date is later without unit", () => {
    expect(
      isAfter(
        new Date(2025, 3, 15, 10, 30, 45, 124),
        new Date(2025, 3, 15, 10, 30, 45, 123),
      ),
    ).toBe(true);
  });

  it("returns false for identical timestamps without unit", () => {
    const date = new Date(2025, 3, 15, 10, 30, 45, 123);
    expect(isAfter(date, new Date(date.getTime()))).toBe(false);
  });

  it("returns false when the date is earlier without unit", () => {
    expect(isAfter(new Date(2025, 3, 15), new Date(2025, 3, 16))).toBe(false);
  });

  it("returns false within the same day when unit is day", () => {
    expect(
      isAfter(new Date(2025, 3, 15, 23, 59), new Date(2025, 3, 15, 1), "day"),
    ).toBe(false);
  });

  it("returns true for the next day when unit is day", () => {
    expect(
      isAfter(
        new Date(2025, 3, 16, 0, 0),
        new Date(2025, 3, 15, 23, 59),
        "day",
      ),
    ).toBe(true);
  });

  it("returns false within the same Sunday-start week", () => {
    expect(isAfter(new Date(2025, 3, 19), new Date(2025, 3, 13), "week")).toBe(
      false,
    );
  });

  it("returns true across a week boundary", () => {
    expect(isAfter(new Date(2025, 3, 20), new Date(2025, 3, 19), "week")).toBe(
      true,
    );
  });

  it("compares at month, quarter and year granularity", () => {
    expect(isAfter(new Date(2025, 4, 1), new Date(2025, 3, 30), "month")).toBe(
      true,
    );
    expect(
      isAfter(new Date(2025, 5, 30), new Date(2025, 3, 1), "quarter"),
    ).toBe(false);
    expect(isAfter(new Date(2026, 0, 1), new Date(2025, 11, 31), "year")).toBe(
      true,
    );
  });

  it("compares at second, minute and hour granularity", () => {
    expect(
      isAfter(
        new Date(2025, 3, 15, 10, 30, 45, 999),
        new Date(2025, 3, 15, 10, 30, 45, 0),
        "second",
      ),
    ).toBe(false);
    expect(
      isAfter(
        new Date(2025, 3, 15, 10, 31),
        new Date(2025, 3, 15, 10, 30, 59),
        "minute",
      ),
    ).toBe(true);
    expect(
      isAfter(new Date(2025, 3, 15, 10, 59), new Date(2025, 3, 15, 10), "hour"),
    ).toBe(false);
  });
});
