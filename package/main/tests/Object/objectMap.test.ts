import { objectMap } from "../../module/Object/objectMap";
test("{objectMap}", () => {
	expect(objectMap({ a: 1, b: 2, c: 3 }, (value) => value * 2)).toEqual(
		expect.objectContaining({ a: 2, b: 4, c: 6 }),
	);
	expect(objectMap({ a: 1, b: 2, c: 3 }, (value) => value * 2)).not.toEqual(
		expect.objectContaining({ a: 1, b: 2, c: 3 }),
	);
});
