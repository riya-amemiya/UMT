import { dayOfWeekSimple } from "@/Simple/Tool/dayOfWeekSimple";
test("dayOfWeekSimple", () => {
  expect(dayOfWeekSimple("2019-01-01")).toBe(2);
  expect(dayOfWeekSimple("2020/01/02")).toBe(4);
});
