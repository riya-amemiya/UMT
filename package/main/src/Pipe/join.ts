/**
Creates a pipe operator that joins array elements into a string
@param {string} [separator] - The string placed between elements, "," when omitted
@returns {(array: readonly unknown[]) => string} An operator that returns the joined string
@example
pipe(["a", "b", "c"], join("-")); // "a-b-c"
pipe(["a", "b", "c"], join()); // "a,b,c"
*/
export const join =
  (separator?: string): ((array: readonly unknown[]) => string) =>
  (array: readonly unknown[]): string =>
    array.join(separator);
