/**
Removes the leading elements of an array while the predicate returns true
@param {T[]} array The array to scan
@param {(value: T, index: number, array: T[]) => boolean} predicate
  Function evaluated for each element until it first returns false
@returns {T[]} A new array starting at the first element that fails the predicate
@example dropWhile([1, 2, 3, 4, 1], (n) => n < 3); // [3, 4, 1]
*/
export const dropWhile = <T>(
  array: T[],
  predicate: (value: T, index: number, array: T[]) => boolean,
): T[] => {
  const start = array.findIndex(
    (value, index) => !predicate(value, index, array),
  );
  return start === -1 ? [] : array.slice(start);
};
