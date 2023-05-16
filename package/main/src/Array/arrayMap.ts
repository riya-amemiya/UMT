export const arrayMap = <T, U>(
	array: T[],
	callbackfn: (value: T, index: number, rowArray: typeof array) => U,
): U[] => {
	const result: U[] = [];
	for (const [index, value] of array.entries()) {
		result.push(callbackfn(value, index, array));
	}
	return result;
};
