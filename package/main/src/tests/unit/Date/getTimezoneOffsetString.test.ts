import { getTimezoneOffsetString } from "@/Date/getTimezoneOffsetString";

describe("getTimezoneOffsetString", () => {
  it("should handle positive timezone offset", () => {
    const date = new Date("2023-01-01T12:00:00Z");
    date.getTimezoneOffset = () => -480; // UTC+8
    expect(getTimezoneOffsetString(date)).toBe("+08:00");
  });

  it("should handle negative timezone offset", () => {
    const date = new Date("2023-01-01T12:00:00Z");
    date.getTimezoneOffset = () => 300; // UTC-5
    expect(getTimezoneOffsetString(date)).toBe("-05:00");
  });

  it("should handle zero timezone offset", () => {
    const date = new Date("2023-01-01T12:00:00Z");
    date.getTimezoneOffset = () => 0; // UTC
    expect(getTimezoneOffsetString(date)).toBe("+00:00");
  });
});
