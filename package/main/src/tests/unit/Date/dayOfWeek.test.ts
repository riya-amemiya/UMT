import { now } from "@/Date";
import { dayOfWeek } from "@/Date/dayOfWeek";

describe("dayOfWeek function", () => {
  it("should return correct day of week for a given date", () => {
    expect(dayOfWeek({ year: 2020, mon: 1, day: 1 })).toBe(3);
    expect(dayOfWeek({ year: 2020, mon: 1, day: 2 })).toBe(4);
  });

  it("should use current year when year is not provided", () => {
    const nowDate = now();
    nowDate.setMonth(0);
    nowDate.setDate(2);
    expect(dayOfWeek({ mon: 1, day: 2 })).toBe(nowDate.getDay());
  });

  it("should use current year and month when both are not provided", () => {
    const nowDate = now();
    nowDate.setDate(2);
    expect(dayOfWeek({ day: 2 })).toBe(nowDate.getDay());
  });

  it("should use current date when no parameters are provided", () => {
    const nowDate = now();
    expect(dayOfWeek({})).toBe(nowDate.getDay());
    expect(dayOfWeek()).toBe(now().getDay());
  });
});
