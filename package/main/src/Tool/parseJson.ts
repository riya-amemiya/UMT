/**
 * Parses a JSON string into a typed JavaScript value
 * @template T The expected type of the parsed value (defaults to unknown)
 * @param json JSON string to parse
 * @returns Parsed value of type T
 * @throws {SyntaxError} If the JSON string is invalid
 */
export const parseJson = <T = unknown>(json: string): T => {
  return JSON.parse(json);
};
