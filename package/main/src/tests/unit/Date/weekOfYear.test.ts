import { startOf } from "@/Date/startOf";
import { weekOfYear } from "@/Date/weekOfYear";

describe("weekOfYear", () => {
  it("returns 1 for January 1", () => {
    expect(weekOfYear(new Date(2025, 0, 1))).toBe(1);
  });

  it("keeps dates in January 1's Sunday-start week as week 1", () => {
    // 2025-01-01 is Wednesday; week starts 2024-12-29
    expect(weekOfYear(new Date(2025, 0, 4))).toBe(1);
  });

  it("increments on the following Sunday", () => {
    // 2025-01-05 is Sunday
    expect(weekOfYear(new Date(2025, 0, 5))).toBe(2);
  });

  it("matches startOf week boundaries across a year", () => {
    const date = new Date(2025, 3, 16);
    const yearStartWeek = startOf(new Date(2025, 0, 1), "week");
    const currentWeek = startOf(date, "week");
    const expected =
      Math.round(
        (currentWeek.getTime() - yearStartWeek.getTime()) / 86_400_000,
      ) /
        7 +
      1;
    expect(weekOfYear(date)).toBe(expected);
  });
});
