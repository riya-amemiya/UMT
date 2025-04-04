import { dateRange } from "@/Date/dateRange";

describe("dateRange", () => {
  it("should generate array of dates between start and end dates", () => {
    const start = new Date("2025-01-01");
    const end = new Date("2025-01-03");
    const dates = dateRange(start, end);

    expect(dates).toHaveLength(3);
    expect(dates[0]).toEqual(new Date("2025-01-01"));
    expect(dates[1]).toEqual(new Date("2025-01-02"));
    expect(dates[2]).toEqual(new Date("2025-01-03"));
  });

  it("should handle single day range", () => {
    const date = new Date("2025-01-01");
    const dates = dateRange(date, date);

    expect(dates).toHaveLength(1);
    expect(dates[0]).toEqual(new Date("2025-01-01"));
  });

  it("should create new Date instances", () => {
    const start = new Date("2025-01-01");
    const end = new Date("2025-01-01");
    const dates = dateRange(start, end);

    // Modify the original date to ensure we have a new instance
    start.setDate(2);

    expect(dates[0]).not.toBe(start);
    expect(dates[0].getDate()).toBe(1);
  });

  it("should handle month and year transitions", () => {
    const start = new Date("2024-12-30");
    const end = new Date("2025-01-02");
    const dates = dateRange(start, end);

    expect(dates).toHaveLength(4);
    expect(dates[0]).toEqual(new Date("2024-12-30"));
    expect(dates[1]).toEqual(new Date("2024-12-31"));
    expect(dates[2]).toEqual(new Date("2025-01-01"));
    expect(dates[3]).toEqual(new Date("2025-01-02"));
  });
});
