const toLowerCaseString = (value: string): string => value.toLowerCase();

/**
Creates a pipe operator that converts a string to lower case
@returns {(value: string) => string} An operator that returns the lower-cased string
@example
pipe("Hello", toLowerCase()); // "hello"
*/
export const toLowerCase = (): ((value: string) => string) => toLowerCaseString;
