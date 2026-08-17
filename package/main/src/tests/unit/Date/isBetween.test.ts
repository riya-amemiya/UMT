import { isBetween } from "@/Date/isBetween";

describe("isBetween", () => {
  const start = new Date(2025, 3, 10, 10, 0, 0, 0);
  const mid = new Date(2025, 3, 15, 12, 0, 0, 0);
  const end = new Date(2025, 3, 20, 18, 0, 0, 0);

  it("returns true for a timestamp strictly inside the range", () => {
    expect(isBetween(mid, start, end)).toBe(true);
  });

  it("returns false for the start bound with default exclusivity", () => {
    expect(isBetween(start, start, end)).toBe(false);
  });

  it("returns false for the end bound with default exclusivity", () => {
    expect(isBetween(end, start, end)).toBe(false);
  });

  it("includes both bounds when inclusivity is []", () => {
    expect(isBetween(start, start, end, undefined, "[]")).toBe(true);
    expect(isBetween(end, start, end, undefined, "[]")).toBe(true);
  });

  it("includes start and excludes end when inclusivity is [)", () => {
    expect(isBetween(start, start, end, undefined, "[)")).toBe(true);
    expect(isBetween(end, start, end, undefined, "[)")).toBe(false);
  });

  it("excludes start and includes end when inclusivity is (]", () => {
    expect(isBetween(start, start, end, undefined, "(]")).toBe(false);
    expect(isBetween(end, start, end, undefined, "(]")).toBe(true);
  });

  it("returns false when start is after end", () => {
    expect(isBetween(mid, end, start)).toBe(false);
  });

  it("returns true for same day at day granularity", () => {
    expect(
      isBetween(
        new Date(2025, 3, 15, 23, 59),
        new Date(2025, 3, 15, 0, 0),
        new Date(2025, 3, 15, 1, 0),
        "day",
        "[]",
      ),
    ).toBe(true);
  });

  it("returns false for adjacent days at day granularity", () => {
    expect(
      isBetween(
        new Date(2025, 3, 16),
        new Date(2025, 3, 10),
        new Date(2025, 3, 15),
        "day",
        "[]",
      ),
    ).toBe(false);
  });

  it("uses Sunday-start week boundaries", () => {
    // 2025-04-13 Sunday through 2025-04-19 Saturday are one week
    expect(
      isBetween(
        new Date(2025, 3, 19),
        new Date(2025, 3, 13),
        new Date(2025, 3, 19),
        "week",
        "[]",
      ),
    ).toBe(true);
    expect(
      isBetween(
        new Date(2025, 3, 20),
        new Date(2025, 3, 13),
        new Date(2025, 3, 19),
        "week",
        "[]",
      ),
    ).toBe(false);
  });

  it("uses quarter granularity", () => {
    expect(
      isBetween(
        new Date(2025, 5, 30),
        new Date(2025, 3, 1),
        new Date(2025, 5, 1),
        "quarter",
        "[]",
      ),
    ).toBe(true);
    expect(
      isBetween(
        new Date(2025, 6, 1),
        new Date(2025, 3, 1),
        new Date(2025, 5, 1),
        "quarter",
        "[]",
      ),
    ).toBe(false);
  });
});
