/**
 * Unwraps a value that may be undefined or null
 * @template T The type of the value to unwrap
 * @param value The value to unwrap (may be undefined or null)
 * @param _message Unused (kept for API compatibility)
 * @returns The unwrapped value of type T
 */
export const unwrap = <T>(value: T | undefined | null, _message: string): T => {
  return value as T;
};
