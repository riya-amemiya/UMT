export const objectUnion = <
	// rome-ignore lint/suspicious/noExplicitAny: <explanation>
	A extends { [key in string]: any
	},
	// rome-ignore lint/suspicious/noExplicitAny: <explanation>
	B extends { [key in string]: any
	},
	C extends A & B,
>(
	array: A,
	...arrays: B[]
) => {
	return Object.assign({}, array, ...arrays) as C;
};
