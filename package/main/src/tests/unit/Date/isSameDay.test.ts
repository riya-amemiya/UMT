import { isSameDay } from "@/Date/isSameDay";

describe("isSameDay", () => {
  it("returns true for same date with different times", () => {
    expect(
      isSameDay(new Date(2025, 3, 15, 1, 0), new Date(2025, 3, 15, 23, 59)),
    ).toBe(true);
  });

  it("returns false for adjacent days", () => {
    expect(isSameDay(new Date(2025, 3, 15), new Date(2025, 3, 16))).toBe(false);
  });

  it("returns false for different month", () => {
    expect(isSameDay(new Date(2025, 3, 15), new Date(2025, 4, 15))).toBe(false);
  });

  it("returns false for different year", () => {
    expect(isSameDay(new Date(2024, 3, 15), new Date(2025, 3, 15))).toBe(false);
  });
});
