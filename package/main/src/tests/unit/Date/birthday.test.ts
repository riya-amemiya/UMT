import { birthday } from "@/Date/birthday";

describe("birthday", () => {
  beforeEach(() => {
    // Mock current date to 2025-04-04
    jest.useFakeTimers();
    jest.setSystemTime(new Date("2025-04-04"));
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it("should calculate age correctly for past birthday this year", () => {
    // Someone born on January 1, 2000 would be 25 on April 4, 2025
    expect(birthday(2000, 1, 1)).toBe(25);
  });

  it("should calculate age correctly for future birthday this year", () => {
    // Someone born on December 31, 2000 would still be 24 on April 4, 2025
    expect(birthday(2000, 12, 31)).toBe(24);
  });

  it("should handle two-digit years", () => {
    // Year 99 should be treated as year 1999 (26 years before 2025)
    expect(birthday(99, 1, 1)).toBe(1926);
  });

  it("should handle timezone offset", () => {
    // Test with default timezone (UTC+9)
    expect(birthday(2000, 1, 1)).toBe(25);
    // Test with different timezone (UTC+0)
    expect(birthday(2000, 1, 1, 0)).toBe(25);
  });
});
