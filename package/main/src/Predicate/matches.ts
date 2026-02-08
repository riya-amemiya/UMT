/**
 * Creates a predicate that checks whether an object matches
 * all properties of the given pattern using strict equality
 * @param {Record<string, unknown>} pattern - The pattern to match against
 * @returns {(obj: Record<string, unknown>) => boolean} A predicate that tests objects
 * @example
 * const isAdmin = matches({ role: "admin" });
 * isAdmin({ name: "Alice", role: "admin" }); // true
 * isAdmin({ name: "Bob", role: "user" }); // false
 */
export const matches =
  (
    pattern: Record<string, unknown>,
  ): ((object: Record<string, unknown>) => boolean) =>
  (object: Record<string, unknown>): boolean => {
    const keys = Object.keys(pattern);
    for (const key of keys) {
      if (object[key] !== pattern[key]) {
        return false;
      }
    }
    return true;
  };
