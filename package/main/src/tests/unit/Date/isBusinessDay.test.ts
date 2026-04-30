import { isBusinessDay } from "@/Date/isBusinessDay";

describe("isBusinessDay", () => {
  it("returns true for Monday with no holidays", () => {
    expect(isBusinessDay(new Date(2025, 3, 21))).toBe(true);
  });

  it("returns false for Saturday", () => {
    expect(isBusinessDay(new Date(2025, 3, 19))).toBe(false);
  });

  it("returns false for Sunday", () => {
    expect(isBusinessDay(new Date(2025, 3, 20))).toBe(false);
  });

  it("returns false for weekday listed as holiday", () => {
    const date = new Date(2025, 3, 21);
    const holidays = [new Date(2025, 3, 21, 12, 0)];
    expect(isBusinessDay(date, holidays)).toBe(false);
  });

  it("returns true for weekday not in holiday list", () => {
    const date = new Date(2025, 3, 22);
    const holidays = [new Date(2025, 3, 21)];
    expect(isBusinessDay(date, holidays)).toBe(true);
  });
});
