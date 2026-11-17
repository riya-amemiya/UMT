const trimString = (value: string): string => value.trim();

/**
Creates a pipe operator that removes leading and trailing whitespace from a string
@returns {(value: string) => string} An operator that returns the trimmed string
@example
pipe("  hello  ", trim()); // "hello"
*/
export const trim = (): ((value: string) => string) => trimString;
