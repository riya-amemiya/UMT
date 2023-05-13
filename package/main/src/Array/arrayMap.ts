export const arrayMap = <T, U>(
	array: T[],
	callbackfn: (value: T, index: number, rowArray: typeof array) => U,
): U[] => {
	const result: U[] = [];
	array.forEach((value, index, array) => {
		result.push(callbackfn(value, index, array));
	});
	return result;
};
