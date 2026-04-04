// Security: Keys that must be stripped during JSON parsing to prevent
// prototype pollution. If an attacker controls the JSON input, they can
// embed keys like "__proto__" which, when the parsed object is later
// spread or merged (e.g. via Object.assign, spread operator, or any
// deep-merge utility), can overwrite properties on Object.prototype
// and affect all objects in the runtime.
const DANGEROUS_KEYS = new Set(["__proto__", "constructor", "prototype"]);

/**
 * Parses a JSON string into a typed JavaScript value
 * @template T The expected type of the parsed value (defaults to unknown)
 * @param json JSON string to parse
 * @returns Parsed value of type T
 * @throws {SyntaxError} If the JSON string is invalid
 */
export const parseJson = <T = unknown>(json: string): T => {
  return JSON.parse(json, (key, value) => {
    if (DANGEROUS_KEYS.has(key)) {
      return;
    }
    return value;
  });
};
