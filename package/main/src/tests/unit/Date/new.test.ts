import { newDateInt, newDateString } from "@/Date/new";

describe("newDateInt", () => {
  beforeEach(() => {
    // Mock timezone to UTC+9 (JST)
    jest.useFakeTimers();
    const mockDate = new Date(2025, 0, 1);
    mockDate.getTimezoneOffset = () => -540; // UTC+9
    jest.setSystemTime(mockDate);
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it("should create date with default time values", () => {
    const date = newDateInt(2025, 1, 1);
    expect(date.getFullYear()).toBe(2025);
    expect(date.getMonth()).toBe(0);
    expect(date.getDate()).toBe(1);
    expect(date.getHours()).toBe(9);
    expect(date.getMinutes()).toBe(0);
    expect(date.getSeconds()).toBe(0);
    expect(date.getMilliseconds()).toBe(0);
  });

  it("should create date with all parameters", () => {
    const date = newDateInt(2025, 1, 1, 10, 30, 45, 500);
    expect(date.getFullYear()).toBe(2025);
    expect(date.getMonth()).toBe(0);
    expect(date.getDate()).toBe(1);
    expect(date.getHours()).toBe(10);
    expect(date.getMinutes()).toBe(30);
    expect(date.getSeconds()).toBe(45);
    expect(date.getMilliseconds()).toBe(500);
  });
});

describe("newDateString", () => {
  it("should create date with default time values", () => {
    const date = newDateString("2025-01-01");
    // Use toISOString to check the UTC time, which should be midnight
    expect(date.toISOString()).toBe("2025-01-01T00:00:00.000Z");
  });

  it("should create date with specific time components", () => {
    const date = newDateString("2025-01-01", "10", "30", "45", "500", "09");
    expect(date.toISOString()).toBe("2025-01-01T01:30:45.500Z"); // UTC+9 -> UTC
  });

  it("should handle different timezone offsets", () => {
    const dateUTC = newDateString("2025-01-01", "12", "00", "00", "000", "00");
    const dateJST = newDateString("2025-01-01", "12", "00", "00", "000", "09");

    // JST (UTC+9) date should be 9 hours behind UTC date
    expect(dateJST.getTime()).toBe(dateUTC.getTime() - 9 * 60 * 60 * 1000);
  });
});
