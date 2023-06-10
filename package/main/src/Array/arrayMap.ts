export const arrayMap = <T, U>(
  array: T[],
  callbackfn: (value: T, index: number, rowArray: typeof array) => U,
): U[] => {
<<<<<<< HEAD
  const result: U[] = [];
  for (const value of array) {
    result.push(callbackfn(value, array.indexOf(value), array));
  }
  return result;
=======
	const result: U[] = [];
	for (const [index, value] of array.entries()) {
		result.push(callbackfn(value, index, array));
	}
	return result;
>>>>>>> origin/main
};
