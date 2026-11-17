const toUpperCaseString = (value: string): string => value.toUpperCase();

/**
Creates a pipe operator that converts a string to upper case
@returns {(value: string) => string} An operator that returns the upper-cased string
@example
pipe("Hello", toUpperCase()); // "HELLO"
*/
export const toUpperCase = (): ((value: string) => string) => toUpperCaseString;
