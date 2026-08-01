import { isSame } from "@/Date/isSame";

describe("isSame", () => {
  it("returns true for identical timestamps without unit", () => {
    const date = new Date(2025, 3, 15, 10, 30, 45, 123);
    expect(isSame(date, new Date(date.getTime()))).toBe(true);
  });

  it("returns false for different milliseconds without unit", () => {
    expect(
      isSame(
        new Date(2025, 3, 15, 10, 30, 45, 123),
        new Date(2025, 3, 15, 10, 30, 45, 124),
      ),
    ).toBe(false);
  });

  it("returns true for same second ignoring milliseconds", () => {
    expect(
      isSame(
        new Date(2025, 3, 15, 10, 30, 45, 0),
        new Date(2025, 3, 15, 10, 30, 45, 999),
        "second",
      ),
    ).toBe(true);
  });

  it("returns false for different seconds", () => {
    expect(
      isSame(
        new Date(2025, 3, 15, 10, 30, 45),
        new Date(2025, 3, 15, 10, 30, 46),
        "second",
      ),
    ).toBe(false);
  });

  it("returns true for same minute ignoring seconds", () => {
    expect(
      isSame(
        new Date(2025, 3, 15, 10, 30, 0),
        new Date(2025, 3, 15, 10, 30, 59),
        "minute",
      ),
    ).toBe(true);
  });

  it("returns true for same hour ignoring minutes", () => {
    expect(
      isSame(
        new Date(2025, 3, 15, 10, 0),
        new Date(2025, 3, 15, 10, 59),
        "hour",
      ),
    ).toBe(true);
  });

  it("returns true for same day with different times", () => {
    expect(
      isSame(new Date(2025, 3, 15, 1, 0), new Date(2025, 3, 15, 23, 59), "day"),
    ).toBe(true);
  });

  it("returns false for adjacent days", () => {
    expect(isSame(new Date(2025, 3, 15), new Date(2025, 3, 16), "day")).toBe(
      false,
    );
  });

  it("returns true for same week Sunday-start", () => {
    // 2025-04-13 is Sunday, 2025-04-19 is Saturday
    expect(isSame(new Date(2025, 3, 13), new Date(2025, 3, 19), "week")).toBe(
      true,
    );
  });

  it("returns false across week boundary", () => {
    // 2025-04-19 Saturday, 2025-04-20 Sunday
    expect(isSame(new Date(2025, 3, 19), new Date(2025, 3, 20), "week")).toBe(
      false,
    );
  });

  it("returns true for same month with different days", () => {
    expect(isSame(new Date(2025, 3, 1), new Date(2025, 3, 30), "month")).toBe(
      true,
    );
  });

  it("returns false for different months", () => {
    expect(isSame(new Date(2025, 3, 15), new Date(2025, 4, 15), "month")).toBe(
      false,
    );
  });

  it("returns true for same quarter", () => {
    // Q2: Apr-Jun
    expect(isSame(new Date(2025, 3, 1), new Date(2025, 5, 30), "quarter")).toBe(
      true,
    );
  });

  it("returns false for different quarters", () => {
    // Mar is Q1, Apr is Q2
    expect(isSame(new Date(2025, 2, 31), new Date(2025, 3, 1), "quarter")).toBe(
      false,
    );
  });

  it("returns true for same year with different months", () => {
    expect(isSame(new Date(2025, 0, 1), new Date(2025, 11, 31), "year")).toBe(
      true,
    );
  });

  it("returns false for different years", () => {
    expect(isSame(new Date(2024, 3, 15), new Date(2025, 3, 15), "year")).toBe(
      false,
    );
  });

  it("matches isSameDay semantics when unit is day", () => {
    const left = new Date(2025, 3, 15, 1, 0);
    const right = new Date(2025, 3, 15, 23, 59);
    expect(isSame(left, right, "day")).toBe(true);
  });
});
