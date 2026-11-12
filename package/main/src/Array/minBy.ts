/**
Returns the element whose selected value is the smallest
Values are compared with `<`, so the first element wins ties and a key that
never compares as smaller (such as NaN) never replaces the current minimum.
@param {T[]} array The array to scan
@param {(value: T) => K} selector Function returning the value to compare
@returns {T | undefined} The element with the smallest selected value, or undefined for an empty array
@example minBy([{ n: 2 }, { n: 1 }, { n: 1 }], (item) => item.n); // { n: 1 } (the second element)
*/
export const minBy = <T, K>(
  array: T[],
  selector: (value: T) => K,
): T | undefined => {
  const length = array.length;
  if (length === 0) {
    return undefined;
  }
  let best = array[0];
  let bestKey = selector(best);
  for (let index = 1; index < length; index++) {
    const value = array[index];
    const key = selector(value);
    if (!(key < bestKey)) {
      continue;
    }
    best = value;
    bestKey = key;
  }
  return best;
};
