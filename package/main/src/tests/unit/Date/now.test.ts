import { now } from "@/Date/now";

describe("now", () => {
  const FIXED_TIME = 1_712_192_400_000; // 2024-04-04T01:00:00.000Z (confirmed with Date object)

  beforeEach(() => {
    // Mock Date.now() to return a fixed timestamp
    jest.spyOn(Date, "now").mockImplementation(() => FIXED_TIME);
  });

  afterEach(() => {
    jest.restoreAllMocks();
  });

  it("should return current time in JST by default", () => {
    const result = now();
    // Expected: 2024-04-04T10:00:00.000Z (FIXED_TIME + 9 hours)
    expect(result.toISOString()).toBe("2024-04-04T10:00:00.000Z");
  });

  it("should return UTC time when offset is 0", () => {
    const result = now(0);
    // Expected: 2024-04-04T01:00:00.000Z (FIXED_TIME + 0 hours)
    expect(result.toISOString()).toBe("2024-04-04T01:00:00.000Z");
  });

  it("should handle different UTC offsets", () => {
    // Test UTC+1 (CET)
    const resultCET = now(1);
    expect(resultCET.toISOString()).toBe("2024-04-04T02:00:00.000Z");

    // Test UTC+8 (CST)
    const resultCST = now(8);
    expect(resultCST.toISOString()).toBe("2024-04-04T09:00:00.000Z");
  });

  it("should return correct time difference between timezones", () => {
    const utcResult = now(0);
    const jstResult = now(9);

    // JST should be 9 hours ahead of UTC
    expect(jstResult.getTime() - utcResult.getTime()).toBe(9 * 60 * 60 * 1000);
  });
});
