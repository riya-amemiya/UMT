export const objectMap = <T, U>(
	object: { [key: string]: T },
	callbackfn: (value: T, key: string, index: number) => U,
) => {
	return Object.fromEntries(
		Object.entries(object).map(([key, value], index) => {
			return [key, callbackfn(value, key, index)];
		}),
	);
};
