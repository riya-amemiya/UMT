import { dayOfWeek } from "@/Tool/dayOfWeek";

test("dayOfWeek", () => {
  expect(dayOfWeek({ year: 2020, mon: 1, day: 1 })).toBe(3);
  expect(dayOfWeek({ year: 2020, mon: 1, day: 2 })).toBe(4);
});
