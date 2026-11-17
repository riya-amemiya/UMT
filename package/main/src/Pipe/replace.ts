/**
Creates a pipe operator that replaces matches of a pattern in a string
@param {string | RegExp} pattern - The string or regular expression to search for
@param {string} replacement - The string that replaces each match
@returns {(value: string) => string} An operator that returns the replaced string
@example
pipe("a-b-c", replace("-", "+")); // "a+b-c"
pipe("a-b-c", replace(/-/g, "+")); // "a+b+c"
*/
export const replace =
  (
    pattern: string | RegExp,
    replacement: string,
  ): ((value: string) => string) =>
  (value: string): string =>
    value.replace(pattern, replacement);
