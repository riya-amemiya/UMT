export const objectUnion = <
	A extends {
		[key in string]: unknown;
	},
	B extends {
		[key in string]: unknown;
	},
	C extends A & B,
>(
	array: A,
	...arrays: B[]
) => {
	return Object.assign({}, array, ...arrays) as C;
};
