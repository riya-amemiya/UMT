import { roundOf } from "../../module/Math/roundOf";
test("{roundOf", () => {
	expect(roundOf(1.111111111111, 2)).toBe(1.11);
	expect(roundOf(1.111111111111, 3)).toBe(1.111);
	expect(roundOf(1.111111111111, 4)).toBe(1.1111);
	expect(roundOf(1.111111111111, 5)).toBe(1.11111);
	expect(roundOf(1.111111111111, 6)).toBe(1.111111);
	expect(roundOf(1.111111111111, 7)).toBe(1.1111111);
	expect(roundOf(1.111111111111, 8)).toBe(1.11111111);
	expect(roundOf(1.555555555555, 2)).toBe(1.56);
});
