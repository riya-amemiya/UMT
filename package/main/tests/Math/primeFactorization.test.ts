import { primeFactorization } from "../../module/Math/primeFactorization";
test("{primeFactorization}", () => {
	const result = primeFactorization(100);
	for (const item of result) {
		if (item.number === 2) {
			expect(item.count).toBe(2);
		} else if (item.number === 5) {
			expect(item.count).toBe(2);
		}
	}
});
