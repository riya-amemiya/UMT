export const arrayMap = <T extends unknown[], U>(
  array: T,
  callbackfn: (value: T[number], index: number, rowArray: typeof array) => U,
): U[] => {
  const result: U[] = [];
  for (const value of array) {
    result.push(callbackfn(value, array.indexOf(value), array));
  }
  return result;
};
