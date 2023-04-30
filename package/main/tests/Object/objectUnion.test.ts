import { objectUnion } from "../../module/Object/objectUnion";
test("{objectUnion}", () => {
	expect(objectUnion({ a: 1 }, { b: 2 })).toEqual({ a: 1, b: 2 });
	expect(objectUnion({ a: 1 }, { a: 2 })).toEqual({ a: 2 });
	expect(objectUnion({ a: 1 }, { a: 2 }, { a: 3 })).toEqual({ a: 3 });
	expect(objectUnion({ a: 1, b: 2 }, { c: 3 }, { d: 4 })).toEqual({
		a: 1,
		b: 2,
		c: 3,
		d: 4,
	});
});
