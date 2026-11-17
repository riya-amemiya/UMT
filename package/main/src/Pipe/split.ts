/**
Creates a pipe operator that splits a string into an array of substrings
@param {string | RegExp} separator - The pattern describing where each split occurs
@param {number} [limit] - The maximum number of substrings to return
@returns {(value: string) => string[]} An operator that returns the substrings
@example
pipe("a,b,c", split(",")); // ["a", "b", "c"]
pipe("a,b,c", split(",", 2)); // ["a", "b"]
*/
export const split =
  (separator: string | RegExp, limit?: number): ((value: string) => string[]) =>
  (value: string): string[] =>
    value.split(separator, limit);
