import { nPr } from "./nPr";
/**
 * nCr
 * @param  {number} n
 * @param  {number} r
 */
export const nCr = (n: number, r: number) => {
	//nCr
	let y = nPr(n, r);
	let age = 1;
	for (let i = 2; i <= r; i++) {
		age *= i;
	}
	y /= age;
	if (1 > y) {
		return 0;
	}
	return y;
};
