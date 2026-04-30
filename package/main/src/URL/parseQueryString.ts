/**
 * Parses a query string into a key-value record.
 *
 * Accepts either a full URL or a raw query string
 * (with or without leading "?").
 *
 * @param query - The query string or URL to parse
 * @returns A record of key-value pairs from the query string
 *
 * @remarks
 * **Prototype pollution warning:** This function does not filter out
 * prototype-polluting keys (`__proto__`, `constructor`, `prototype`).
 * If processing user-controlled input, sanitize the result with
 * `removePrototype` after calling this function.
 *
 * @example
 * ```typescript
 * parseQueryString("?page=1&q=search");
 * // { page: "1", q: "search" }
 *
 * parseQueryString("foo=bar&baz=qux");
 * // { foo: "bar", baz: "qux" }
 *
 * parseQueryString("https://example.com?a=1&b=2");
 * // { a: "1", b: "2" }
 * ```
 */
export const parseQueryString = (query: string): Record<string, string> => {
  let searchString = query;
  if (query.includes("://")) {
    searchString = new URL(query).search;
  }

  const parameters = new URLSearchParams(searchString);
  const result: Record<string, string> = {};
  for (const [key, value] of parameters) {
    result[key] = value;
  }
  return result;
};
