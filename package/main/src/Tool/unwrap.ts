/**
 * Unwraps a value that may be undefined or null, throwing an error if the value is absent
 * @template T The type of the value to unwrap
 * @param value The value to unwrap (may be undefined or null)
 * @param message The error message to throw if the value is absent
 * @returns The unwrapped value of type T
 * @throws {Error} If the value is undefined or null
 */
export const unwrap = <T>(value: T | undefined | null, message: string): T => {
  if (value !== undefined && value !== null) {
    return value;
  }
  throw new Error(message);
};
