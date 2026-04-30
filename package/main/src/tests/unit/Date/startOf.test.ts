import { startOf } from "@/Date/startOf";

describe("startOf", () => {
  it("zeros milliseconds for second", () => {
    const result = startOf(new Date(2025, 3, 15, 10, 30, 45, 123), "second");
    expect(result.getMilliseconds()).toBe(0);
    expect(result.getSeconds()).toBe(45);
  });

  it("zeros seconds for minute", () => {
    const result = startOf(new Date(2025, 3, 15, 10, 30, 45, 123), "minute");
    expect(result.getSeconds()).toBe(0);
    expect(result.getMilliseconds()).toBe(0);
  });

  it("zeros minutes for hour", () => {
    const result = startOf(new Date(2025, 3, 15, 10, 30, 45, 123), "hour");
    expect(result.getMinutes()).toBe(0);
    expect(result.getSeconds()).toBe(0);
    expect(result.getMilliseconds()).toBe(0);
  });

  it("zeros hours for day", () => {
    const result = startOf(new Date(2025, 3, 15, 10, 30), "day");
    expect(result.getHours()).toBe(0);
    expect(result.getMinutes()).toBe(0);
    expect(result.getSeconds()).toBe(0);
    expect(result.getMilliseconds()).toBe(0);
  });

  it("returns Sunday for week", () => {
    const result = startOf(new Date(2025, 3, 16), "week");
    expect(result.getDay()).toBe(0);
    expect(result.getHours()).toBe(0);
  });

  it("returns first day for month", () => {
    const result = startOf(new Date(2025, 3, 15), "month");
    expect(result.getDate()).toBe(1);
    expect(result.getMonth()).toBe(3);
    expect(result.getHours()).toBe(0);
  });

  it("returns first day of quarter for January date", () => {
    const result = startOf(new Date(2025, 0, 31), "quarter");
    expect(result.getMonth()).toBe(0);
    expect(result.getDate()).toBe(1);
  });

  it("returns first day of quarter for May date", () => {
    const result = startOf(new Date(2025, 4, 31), "quarter");
    expect(result.getMonth()).toBe(3);
    expect(result.getDate()).toBe(1);
  });

  it("returns first day of quarter for August date", () => {
    const result = startOf(new Date(2025, 7, 31), "quarter");
    expect(result.getMonth()).toBe(6);
    expect(result.getDate()).toBe(1);
  });

  it("returns first day of quarter for November date", () => {
    const result = startOf(new Date(2025, 10, 30), "quarter");
    expect(result.getMonth()).toBe(9);
    expect(result.getDate()).toBe(1);
  });

  it("returns Jan 1 for year", () => {
    const result = startOf(new Date(2025, 5, 15), "year");
    expect(result.getMonth()).toBe(0);
    expect(result.getDate()).toBe(1);
    expect(result.getHours()).toBe(0);
  });

  it("returns the date unchanged for an unknown unit", () => {
    const input = new Date(2025, 3, 15, 10, 30);
    // @ts-expect-error
    const result = startOf(input, "unknown");
    expect(result.getTime()).toBe(input.getTime());
  });
});
