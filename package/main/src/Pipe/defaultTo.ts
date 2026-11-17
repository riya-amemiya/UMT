/**
Creates a pipe operator that replaces null or undefined with a default value
@param {D} defaultValue - The value returned when the input is null or undefined
@returns {<T>(value: T) => NonNullable<T> | D} An operator that returns the input or the default value
@example
pipe(undefined, defaultTo("")); // ""
pipe("text", defaultTo("")); // "text"
*/
export const defaultTo =
  <D>(defaultValue: D): (<T>(value: T) => NonNullable<T> | D) =>
  <T>(value: T): NonNullable<T> | D =>
    value ?? defaultValue;
