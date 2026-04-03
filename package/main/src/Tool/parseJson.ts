// Security: Keys that must be stripped during JSON parsing to prevent
// prototype pollution. When the parsed result is later spread into
// another object or passed to Object.assign / mergeDeep / etc., a
// "__proto__" own-property will overwrite Object.prototype, allowing
// an attacker to inject arbitrary properties into every object in the
// runtime. Stripping these keys at parse time is a defense-in-depth
// measure that neutralises malicious payloads such as
// '{"__proto__":{"isAdmin":true}}'.
const DANGEROUS_KEYS = new Set(["__proto__", "constructor", "prototype"]);

/**
 * Parses a JSON string into a typed JavaScript value.
 *
 * A reviver is used to silently drop keys that could cause prototype
 * pollution (`__proto__`, `constructor`, `prototype`) so that the
 * returned value is safe to spread or merge into other objects.
 *
 * @template T The expected type of the parsed value (defaults to unknown)
 * @param json JSON string to parse
 * @returns Parsed value of type T
 * @throws {SyntaxError} If the JSON string is invalid
 */
export const parseJson = <T = unknown>(json: string): T => {
  return JSON.parse(json, (key, value) => {
    if (DANGEROUS_KEYS.has(key)) {
      return undefined;
    }
    return value;
  });
};
