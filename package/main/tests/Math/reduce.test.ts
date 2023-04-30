import { reduce } from "../../module/Math/reduce";
test("{reduce}", () => {
	expect(reduce(16, 2)).toMatchObject({
		x: 8,
		y: 1,
		gcd: 2,
	});
});
