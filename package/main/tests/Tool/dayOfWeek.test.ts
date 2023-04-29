import { dayOfWeek } from "../../module/Tool/dayOfWeek";
test("dayOfWeek", () => {
	expect(dayOfWeek({ yer: 2020, mon: 1, day: 1 })).toBe(3);
	expect(dayOfWeek({ yer: 2020, mon: 1, day: 2 })).toBe(4);
});
