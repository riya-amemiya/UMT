/**
Returns the leading elements of an array while the predicate returns true
@param {T[]} array The array to scan
@param {(value: T, index: number, array: T[]) => boolean} predicate
  Function evaluated for each element until it first returns false
@returns {T[]} A new array of the leading elements that satisfy the predicate
@example takeWhile([1, 2, 3, 4, 1], (n) => n < 3); // [1, 2]
*/
export const takeWhile = <T>(
  array: T[],
  predicate: (value: T, index: number, array: T[]) => boolean,
): T[] => {
  const end = array.findIndex(
    (value, index) => !predicate(value, index, array),
  );
  return end === -1 ? array.slice() : array.slice(0, end);
};
