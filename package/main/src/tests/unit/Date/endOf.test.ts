import { endOf } from "@/Date/endOf";

describe("endOf", () => {
  it("sets milliseconds to 999 for second", () => {
    const result = endOf(new Date(2025, 3, 15, 10, 30, 45, 0), "second");
    expect(result.getMilliseconds()).toBe(999);
    expect(result.getSeconds()).toBe(45);
  });

  it("sets to 59.999 for minute", () => {
    const result = endOf(new Date(2025, 3, 15, 10, 30, 0, 0), "minute");
    expect(result.getSeconds()).toBe(59);
    expect(result.getMilliseconds()).toBe(999);
  });

  it("sets to xx:59:59.999 for hour", () => {
    const result = endOf(new Date(2025, 3, 15, 10, 0, 0, 0), "hour");
    expect(result.getMinutes()).toBe(59);
    expect(result.getSeconds()).toBe(59);
    expect(result.getMilliseconds()).toBe(999);
  });

  it("sets to 23:59:59.999 for day", () => {
    const result = endOf(new Date(2025, 3, 15, 0, 0, 0, 0), "day");
    expect(result.getHours()).toBe(23);
    expect(result.getMinutes()).toBe(59);
    expect(result.getSeconds()).toBe(59);
    expect(result.getMilliseconds()).toBe(999);
  });

  it("returns Saturday end for week", () => {
    const result = endOf(new Date(2025, 3, 16), "week");
    expect(result.getDay()).toBe(6);
    expect(result.getHours()).toBe(23);
  });

  it("returns last day for month", () => {
    const result = endOf(new Date(2025, 3, 1), "month");
    expect(result.getDate()).toBe(30);
    expect(result.getMonth()).toBe(3);
  });

  it("returns Feb 28 for non-leap February", () => {
    const result = endOf(new Date(2025, 1, 1), "month");
    expect(result.getDate()).toBe(28);
  });

  it("returns Feb 29 for leap February", () => {
    const result = endOf(new Date(2024, 1, 1), "month");
    expect(result.getDate()).toBe(29);
  });

  it("returns last day of Q1 quarter", () => {
    const result = endOf(new Date(2025, 0, 1), "quarter");
    expect(result.getMonth()).toBe(2);
    expect(result.getDate()).toBe(31);
  });

  it("returns last day of Q2 quarter", () => {
    const result = endOf(new Date(2025, 3, 1), "quarter");
    expect(result.getMonth()).toBe(5);
    expect(result.getDate()).toBe(30);
  });

  it("returns last day of Q3 quarter", () => {
    const result = endOf(new Date(2025, 6, 1), "quarter");
    expect(result.getMonth()).toBe(8);
    expect(result.getDate()).toBe(30);
  });

  it("returns last day of Q4 quarter", () => {
    const result = endOf(new Date(2025, 9, 1), "quarter");
    expect(result.getMonth()).toBe(11);
    expect(result.getDate()).toBe(31);
  });

  it("returns Dec 31 for year", () => {
    const result = endOf(new Date(2025, 5, 15), "year");
    expect(result.getMonth()).toBe(11);
    expect(result.getDate()).toBe(31);
    expect(result.getHours()).toBe(23);
  });

  it("returns the date unchanged for an unknown unit", () => {
    const input = new Date(2025, 3, 15, 10, 30);
    // @ts-expect-error
    const result = endOf(input, "unknown");
    expect(result.getTime()).toBe(input.getTime());
  });
});
