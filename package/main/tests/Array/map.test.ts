import { map } from "../../module/Array/map";
test("{map}", () => {
	expect(map([1, 2, 3], (value) => value * 2)).toEqual(
		expect.arrayContaining([2, 4, 6]),
	);
});
