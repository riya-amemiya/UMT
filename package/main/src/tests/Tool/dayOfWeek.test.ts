import { now } from "@/Date";
import { dayOfWeek } from "@/Tool/dayOfWeek";

describe("isArr", () => {
  it("dayOfWeek", () => {
    expect(dayOfWeek({ year: 2020, mon: 1, day: 1 })).toBe(3);
    expect(dayOfWeek({ year: 2020, mon: 1, day: 2 })).toBe(4);
  });

  it("year is undefined", () => {
    const nowDate = now();
    nowDate.setMonth(0);
    nowDate.setDate(2);
    expect(dayOfWeek({ mon: 1, day: 2 })).toBe(nowDate.getDay());
  });

  it("year and mon is undefined", () => {
    const nowDate = now();
    nowDate.setDate(2);
    expect(dayOfWeek({ day: 2 })).toBe(nowDate.getDay());
  });

  it("props is undefined", () => {
    const nowDate = now();
    expect(dayOfWeek({})).toBe(nowDate.getDay());
    expect(dayOfWeek()).toBe(now().getDay());
  });
});
